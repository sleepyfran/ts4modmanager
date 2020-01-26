import downloaderList from './downloaders/index';
import { some } from 'lodash';

export const NotAnUrl = 0;
export const NotAvailable = 1;
export const UnrecognizedSite = 2;
export const FoundDownloader = 3;

const UrlValidationRegex = /https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)/;

/**
 * Checks whether the given URL matches against any of the given matchers.
 * @param {string} url URL to check.
 * @param {string[]} matchers List of matching URLs.
 */
const urlMatches = (url, matchers) => {
    const urlHost = new URL(url).host;

    return some(matchers, matchingUrl => {
        const matchingUrlHost = new URL(matchingUrl).host;
        return urlHost === matchingUrlHost;
    });
};

/**
 * Finds a downloader for the given URL.
 * @param {string} url URL of the mod to download.
 * @returns {[number, object]} An array containing a first element with the result of the download (use the const
 * defined in this file), and the associated downloader, if any.
 */
export const findDownloader = url => {
    const isValidUrl = UrlValidationRegex.test(url);
    if (!isValidUrl) return [NotAnUrl, undefined];

    const matchingDownloader = downloaderList.find(d =>
        urlMatches(url, d.matchers),
    );
    if (!matchingDownloader) return [UnrecognizedSite, undefined];

    return [FoundDownloader, matchingDownloader];
};

/**
 * Attempts to transform the URL content into our object describing the mod state.
 * @param {*} url URL of the mod to download.
 * @param {*} downloader Downloader to use.
 * @returns {[number, object]} An array containing a first element with the result of the parsing (use the const
 * defined in this file), and the information of the mod, if any.
 */
export const parseUrlContent = (url, downloader) => {
    return [0, {}];
};
