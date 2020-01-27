import ModTheSimsDownloader from './modthesims';

/**
 * @typedef {Object} DateInformation
 * @property {string} dateFormat Format of the dates to parse. Should not have any whitespace in the format since they
 * are removed from the original string.
 * @property {string} dateLocale Locale of the date format.
 * @property {string} createdSelector HTML selector to a text that can indicate when the mod was uploaded/created.
 * @property {string} updatedSelector HTML selector to a text that can indicate when the mod was updated.
 * @property {boolean} containedInSameElement Defines whether the downloader needs special handling for cases like in
 * Mod The Sims where the created and the updated date are both within one HTML element.
 * @property {string} createdTitle Title that goes before the actual created date. See `containedInSameElement`.
 * @property {string} updatedTitle Title that goes before the actual updated date. See `containedInSameElement`.
 * @property {string} titleSeparator Separator between the update and the create date. See `containedInSameElement`.
 * @property {string} titleEndSeparator Ending separator of the title.
 */

/**
 * @typedef {Object} DownloadInformation
 * @property {string} downloadSelector HTML selector to download the item. This can either be a direct selector to
 * a button (best way if possible) or just to an element that we can try to parse in search of downloadable items. By
 * default the parser will try to get a valid URL from the element and otherwise will just go and search for URLs in
 * the children of this element. Use with `containedInTable`.
 * @property {boolean} containedInTable Indicates whether the given `downloadSelector` is a table or a direct download
 * element. If it's a table the downloader will check if it has more than one row, in which case it'll parse all the
 * rows and present the user with a selection to download an specific element or all of them. If it only has one row
 * then it'll download that item.
 */

/**
 * Defines a downloader that can take care of automatically downloading a mod given an URL. All the downloaders are
 * just the definition of a set of HTML selectors and rules and should not have any logic attached to them to make
 * them as easier to create as possible.
 * @typedef {Object} Downloader
 * @property {string} name Name of the downloader.
 * @property {string[]} matchers URLs that can match to this downloader. Should be unique to the downloader.
 * @property {string} modNameSelector HTML selector to the name of the mod.
 * @property {DateInformation} dateInformation Information to fetch the created and updated dates.
 * @property {DownloadInformation} downloadInformation Information to fetch the download URL.
 */
export default [ModTheSimsDownloader];
