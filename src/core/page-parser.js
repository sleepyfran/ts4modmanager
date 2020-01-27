import downloaders, { Downloader } from './downloaders/index';
import moment from 'moment';
import { validUrl } from './util';

/**
 * Removes all the spaces from a given string.
 * @param {string} str String to remove spaces from.
 */
const removeSpaces = str => str.replace(/\s/g, '');

/**
 * Creates a date from the given selector's element's text content.
 * @param {Document} dom DOM of the retrieved page.
 * @param {string} selector Selector of the date to retrieve.
 */
const dateFromSelector = (dom, selector) => {
    const element = dom.querySelector(selector);
    if (!element) return null;

    return new Date(removeSpaces(element.textContent));
};

/**
 * Returns the creation and update date of the mod from the same element. Attempts to parse the text inside of the
 * element to get both.
 * @param {Document} dom DOM of the retrieved page.
 * @param {Downloader} downloader Downloader used to parse the page.
 * @returns {[Date, Date]} Creation and update date of the mod.
 */
const retrieveSameElementDates = (dom, downloader) => {
    const dateElement = dom.querySelector(
        downloader.dateInformation.createdSelector,
    );

    const elementContent = dateElement.textContent;
    const createDateTitleIndex = elementContent.indexOf(
        downloader.dateInformation.createdTitle,
    );
    const separatorIndex = elementContent.indexOf(
        downloader.dateInformation.titleSeparator,
    );
    const updateDateTitleIndex = elementContent.indexOf(
        downloader.dateInformation.updatedTitle,
    );

    const createDateChunk = elementContent.slice(
        createDateTitleIndex + downloader.dateInformation.createdTitle.length,
        separatorIndex,
    );
    const updateDateChunk = elementContent.slice(
        updateDateTitleIndex + downloader.dateInformation.updatedTitle.length,
        elementContent.length,
    );

    if (!createDateChunk || !updateDateChunk) return [null, null];

    const createDate = moment(
        removeSpaces(createDateChunk),
        downloader.dateInformation.dateFormat,
        downloader.dateInformation.dateLocale,
    );
    const updateDate = moment(
        removeSpaces(updateDateChunk),
        downloader.dateInformation.dateFormat,
        downloader.dateInformation.dateLocale,
    );

    return [createDate.toDate(), updateDate.toDate()];
};

/**
 * Returns the creation and update date of the mod from different elements.
 * @param {Document} dom DOM of the retrieved page.
 * @param {Downloader} downloader Downloader used to parse the page.
 * @returns {[Date, Date]} Creation and update date of the mod.
 */
const retrieveDifferentElementDates = (dom, downloader) => {
    const createDate = dateFromSelector(
        dom,
        downloader.dateInformation.createdSelector,
    );
    const updateDate = dateFromSelector(
        dom,
        downloader.dateInformation.updatedSelector,
    );

    return [createDate, updateDate];
};

/**
 * Returns the creation and update date of the mod.
 * @param {Document} dom DOM of the retrieved page.
 * @param {Downloader} downloader Downloader used to parse the page.
 * @returns {[Date, Date]} Creation and update date of the mod.
 */
const retrieveDates = (dom, downloader) => {
    return downloader.dateInformation.containedInSameElement
        ? retrieveSameElementDates(dom, downloader)
        : retrieveDifferentElementDates(dom, downloader);
};

/**
 * Attempts to get a valid URL from an element.
 * @param {Element} element Element where the file is located.
 * @returns {[string, string]} An array with two elements consisting of the name of the file and the URL, or null if
 * nothing was found.
 */
const tryGetFileInfo = element => {
    const hrefAttribute = element.getAttribute('href');
    if (hrefAttribute && validUrl(hrefAttribute))
        return [removeSpaces(element.textContent), removeSpaces(hrefAttribute)];

    return null;
};

/**
 * Returns the file that can be downloaded from a given element. It'll try to fetch a valid URL from the text field,
 * href field or wherever it can find some URL.
 * @param {Element} element Element where the file is located.
 * @returns {FileInfo} The different files that can be downloaded.
 */
const retrieveSingleFile = element => {
    const fileInfo = tryGetFileInfo(element);

    if (!fileInfo) {
        const children = Array.from(element.children);
        if (children.length === 0) return null;

        for (const child of children) {
            const foundFile = retrieveSingleFile(child);
            if (foundFile) return foundFile;
        }

        return null;
    }

    return {
        name: fileInfo[0],
        url: fileInfo[1],
    };
};

/**
 * Returns the file or files that can be downloaded from a table.
 * @param {Element} element Element where the files are located.
 * @returns {FileInfo[]} The different files that can be downloaded.
 */
const retrieveFilesFromTable = element => {
    const rows = Array.from(element.querySelectorAll('tbody > tr > td'));
    return rows.map(retrieveSingleFile).filter(f => f);
};

/**
 * Returns the file or files that can be downloaded.
 * @param {Document} dom DOM of the retrieved page.
 * @param {Downloader} downloader Downloader used to parse the page.
 * @returns {FileInfo[]} The different files that can be downloaded.
 *
 * @typedef FileInfo
 * @property {string} name Name of the file to download.
 * @property {string} url URL to download the mod.
 */
const retrieveFiles = (dom, downloader) => {
    const element = dom.querySelector(
        downloader.downloadInformation.downloadSelector,
    );

    return downloader.downloadInformation.containedInTable
        ? retrieveFilesFromTable(element)
        : [retrieveSingleFile(element)];
};

/**
 * Returns the name of the mod being downloaded.
 * @param {Document} dom DOM of the retrieved page.
 * @param {Downloader} downloader Downloader used to parse the page.
 * @returns {string} Name of the mod.
 */
const retrieveModName = (dom, downloader) => {
    const nameElement = dom.querySelector(downloader.modNameSelector);
    return nameElement.textContent.trim();
};

/**
 * Transforms a DOM into a ModInfo object using the given downloader information.
 * @param {Document} dom DOM of the retrieved page.
 * @param {Downloader} downloader Downloader used to parse the page.
 * @returns {ModInfo} Information of the mod parsed from the page.
 *
 * @typedef ModInfo
 * @property {string} name Name of the mod.
 * @property {FileInfo[]} files Possible files to download.
 * @property {Date} createdOn Date of creation of the mod.
 * @property {Date} updatedOn Date of update of the mod.
 */
export const parsePageContent = (dom, downloader) => {
    const dates = retrieveDates(dom, downloader);
    const files = retrieveFiles(dom, downloader);

    return {
        name: retrieveModName(dom, downloader),
        files,
        createdOn: dates[0],
        updatedOn: dates[1],
    };
};
