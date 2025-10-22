import {
  AUTO_LAUNCH_PERMISSIONS,
  checkNativeStatus,
  launchSite,
  obtainSiteList,
  PREF_AUTO_LAUNCH_EXCLUSION,
  PREF_DISPLAY_PAGE_ACTION,
  PREF_ENABLE_AUTO_LAUNCH,
  PREF_SHOW_UPDATE_POPUP
} from './utils'

// == INSTALL AND UPDATE HANDLING

const updateNotification = 'update-available-notification'

// Display the installation page when extension is installed
// Display the update notification when the extension is updated
browser.runtime.onInstalled.addListener(async ({ reason }) => {
  switch (reason) {
    case 'install':
      await browser.tabs.create({ url: browser.runtime.getURL('setup/install.html') })
      break
    case 'update':
      if (
        (await browser.storage.local.get({ [PREF_SHOW_UPDATE_POPUP]: true }))[PREF_SHOW_UPDATE_POPUP] &&
        (await checkNativeStatus()) !== 'ok'
      ) {
        // We use browser localization here as otherwise the messages would get duplicated
        // See: https://github.com/parcel-bundler/parcel/issues/9446
        await browser.notifications.create(updateNotification, {
          title: browser.i18n.getMessage('updateNotificationTitle'),
          message: browser.i18n.getMessage('updateNotificationMessage'),
          iconUrl: browser.runtime.getURL('images/addon-logo.svg'),
          type: 'basic'
        })
      }
  }
})

// Open the update page when the update notification is clicked
browser.notifications.onClicked.addListener(async notification => {
  if (notification !== updateNotification) return
  await browser.tabs.create({ url: browser.runtime.getURL('setup/update.html') })
})

// == CONTENT SCRIPT HANDLING

// Check if the URL is considered a secure context
// Docs: https://developer.mozilla.org/en-US/docs/Web/Security/Secure_Contexts
function isSecureURL (url) {
  return url.protocol === 'https:' || url.hostname === '127.0.0.1' || url.hostname === 'localhost' || url.hostname.endsWith('.localhost')
}

// Detect manifest sent from content script
browser.runtime.onMessage.addListener(async ({ manifestUrl, documentUrl, isSecureContext }, { tab }) => {
  manifestUrl = manifestUrl ? new URL(manifestUrl) : undefined
  documentUrl = documentUrl ? new URL(documentUrl) : undefined

  // Check status of the native program and hide page action if needed
  switch (await checkNativeStatus()) {
    case 'install':
    case 'update-major':
      await browser.pageAction.hide(tab.id)
      return
  }

  // Check if the web app is loaded over a secure context
  // We do this through checking the URLs for https, or for special hosts (e.g. localhost)
  // We also check the browser implementation of secure context with `isSecureContext`, just in case
  // If both the document and the current page pass these checks, the site is a valid web app
  let isValidPwa = manifestUrl && isSecureURL(manifestUrl) && isSecureURL(documentUrl) && isSecureContext

  // Force show or hide the page action depending on user preference
  const settingsDisplayPageAction = (await browser.storage.local.get(PREF_DISPLAY_PAGE_ACTION))[PREF_DISPLAY_PAGE_ACTION]
  if (settingsDisplayPageAction === 'always') isValidPwa = true
  if (settingsDisplayPageAction === 'never') isValidPwa = false

  if (isValidPwa) {
    // Check if this site is already installed
    const existingSites = Object.values(await obtainSiteList()).map(site => site.config.manifest_url)
    const siteInstalled = manifestUrl && existingSites.includes(manifestUrl.toString())

    // Set popup to the launch/install page depending on if it is installed
    // We use browser localization here as these messages are part of the browser UI
    if (siteInstalled) {
      await browser.pageAction.setIcon({ tabId: tab.id, path: 'images/page-action-launch.svg' })
      browser.pageAction.setTitle({ tabId: tab.id, title: browser.i18n.getMessage('actionLaunchSite') })
      browser.pageAction.setPopup({ tabId: tab.id, popup: 'sites/launch.html' })
    } else {
      await browser.pageAction.setIcon({ tabId: tab.id, path: 'images/page-action-install.svg' })
      browser.pageAction.setTitle({ tabId: tab.id, title: browser.i18n.getMessage('actionInstallSite') })
      browser.pageAction.setPopup({ tabId: tab.id, popup: 'sites/install.html' })
    }

    // Show the page action
    await browser.pageAction.show(tab.id)
  } else {
    // Hide the page action
    await browser.pageAction.hide(tab.id)
  }
})

// == PERMISSION HANDLING

// Reload the extension after auto launch permissions have been added
// Or disable the preference if permissions have been revoked
const permissionsListener = async () => {
  // Disable the preference if permissions are not correct
  const permissionsOk = await browser.permissions.contains(AUTO_LAUNCH_PERMISSIONS)
  if (!permissionsOk) await browser.storage.local.set({ [PREF_ENABLE_AUTO_LAUNCH]: false })

  // Reload the extension so listeners become registered
  const preferenceEnabled = (await browser.storage.local.get(PREF_ENABLE_AUTO_LAUNCH))[PREF_ENABLE_AUTO_LAUNCH]
  if (permissionsOk && preferenceEnabled) browser.runtime.reload()
}

browser.permissions.onAdded.addListener(permissionsListener)
browser.permissions.onRemoved.addListener(permissionsListener)

// == LAUNCH ON WEBSITE HANDLING

// Handle opening new URLs and redirect enable URLs to web apps
// This will obtain site list for every request which will impact performance
// In the future, we should find a way to cache it and only update it when it changes

const getMatchingUrlHandler = async target => {
  target = new URL(target)

  for (const site of Object.values(await obtainSiteList())) {
    if (site.config.enabled_url_handlers?.some(handler =>
      target.origin === new URL(handler).origin &&
      target.pathname.startsWith(new URL(handler).pathname)
    )) {
      return site
    }
  }
}

// Track opened tabs that are new navigation targets
const navigationTargetTabs = new Set()

// Handle top-level GET requests and redirects
browser.webRequest?.onBeforeRequest.addListener(
  async details => {
    // Only handle top-level GET requests
    if (details.type !== 'main_frame' || details.method !== 'GET') return

    // Get auto launch extension settings
    const settings = await browser.storage.local.get([PREF_ENABLE_AUTO_LAUNCH, PREF_AUTO_LAUNCH_EXCLUSION])

    // Only handle when the auto launch feature is enabled
    if (!settings[PREF_ENABLE_AUTO_LAUNCH]) return

    // Do not handle excluded URLs
    const pattern = settings[PREF_AUTO_LAUNCH_EXCLUSION]
    const re = new RegExp(pattern)
    if (pattern && re.test(details.url)) return

    // Find the matching web app
    const site = await getMatchingUrlHandler(details.url)
    if (!site) return

    // Launch the web app on target URL
    await browser.runtime.sendNativeMessage('firefoxpwa', {
      cmd: 'LaunchSite',
      params: { id: site.ulid, url: details.url }
    })

    // Close the tab if it was opened as a new navigation target
    if (navigationTargetTabs.has(details.tabId)) {
      navigationTargetTabs.delete(details.tabId)
      await browser.tabs.remove(details.tabId)
    }

    // Prevent the request from being processed by the browser
    return { cancel: true }
  },
  { urls: ['<all_urls>'] },
  ['blocking']
)

// Track tabs opened as new navigation targets
browser.webNavigation?.onCreatedNavigationTarget.addListener(details => {
  navigationTargetTabs.add(details.tabId)
})

// Remove tab from the set after its first committed navigation
browser.webNavigation?.onCommitted.addListener(details => {
  if (details.frameId === 0) {
    navigationTargetTabs.delete(details.tabId)
  }
})

// Remove tab from the set after it has been closed
browser.tabs.onRemoved.addListener(tabId => {
  navigationTargetTabs.delete(tabId)
})

// = LAUNCH ON BROWSER HANDLING

browser.runtime.onStartup.addListener(async () => {
  for (const site of Object.values(await obtainSiteList())) {
    if (site.config.launch_on_browser) {
      await launchSite(site)
    }
  }
})
