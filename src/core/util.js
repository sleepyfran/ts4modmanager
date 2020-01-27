const UrlValidationRegex = /https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)/;

/**
 * Checks whether the given URL has a valid format or not.
 * @param {string} url URL to check.
 */
export const validUrl = url => UrlValidationRegex.test(url);
