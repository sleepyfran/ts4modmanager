export default {
    name: 'Mod The Sims',
    matchers: ['https://www.modthesims.info'],
    dateInformation: {
        createdSelector:
            'body > div:nth-child(7) > div > div > div.maincontentinner > div.well.profilepic.well-small.well-inline.clearfix > div.pull-left',
        updatedSelector:
            'body > div:nth-child(7) > div > div > div.maincontentinner > div.well.profilepic.well-small.well-inline.clearfix > div.pull-left',
        containedInSameElement: true,
        createdTitle: 'Posted',
        updatedTitle: 'Updated',
        titleSeparator: '-',
    },
    downloadInformation: {
        downloadSelector: '#actualtab1 > div > table',
        containedInTable: true,
    },
};
