export default {
    name: 'Mod The Sims',
    matchers: ['https://www.modthesims.info'],
    modNameSelector:
        'body > div:nth-child(7) > div > div > div.maincontentinner > div.well.profilepic.well-small.well-inline.clearfix > h2',
    dateInformation: {
        dateFormat: 'DoMMMYYYYhh:mmA',
        dateLocale: 'en',
        createdSelector:
            'body > div:nth-child(7) > div > div > div.maincontentinner > div.well.profilepic.well-small.well-inline.clearfix > div.pull-left',
        updatedSelector:
            'body > div:nth-child(7) > div > div > div.maincontentinner > div.well.profilepic.well-small.well-inline.clearfix > div.pull-left',
        containedInSameElement: true,
        createdTitle: 'Posted',
        updatedTitle: 'Updated',
        titleSeparator: '-',
        titleEndSeparator: ' by',
    },
    downloadInformation: {
        downloadSelector: '#actualtab1 > div > table',
        containedInTable: true,
    },
};
