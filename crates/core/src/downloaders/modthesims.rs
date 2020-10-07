use super::{CreateAndUpdateDateSelector, DateInfoSelectors, Downloader, ModInfoSelectors};

#[derive(Default)]
pub struct ModTheSimsDownloader;

impl Downloader for ModTheSimsDownloader {
    fn name(&self) -> String {
        "ModTheSims".into()
    }

    fn hostname_matchers(&self) -> Vec<String> {
        vec!["modthesims.info".into()]
    }

    fn get_info_selectors(&self) -> ModInfoSelectors {
        ModInfoSelectors {
            name: "div.well.profilepic.well-small.well-inline.clearfix > h2".into(),
        }
    }

    fn get_date_selectors(&self) -> DateInfoSelectors {
        DateInfoSelectors {
            format: "DoMMMYYYYhh:mmA".into(),
            locale: "en".into(),
            selector: CreateAndUpdateDateSelector::Same {
                regex: "".into(),
                selector: "div.well.profilepic.well-small.well-inline.clearfix > div:nth-child(5)"
                    .into(),
            },
        }
    }

    fn get_download_selector(&self) -> String {
        "#actualtab1 > table".into()
    }
}
