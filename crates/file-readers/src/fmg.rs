use std::str::FromStr;

use std::path::Path;

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FmgCategories {
    TalkMsg = 1,
    BloodMsg = 2,
    MovieSubtitle = 3,
    GoodsName = 10,
    WeaponName = 11,
    ProtectorName = 12,
    AccessoryName = 13,
    MagicName = 14,
    NpcName = 18,
    PlaceName = 19,
    GoodsInfo = 20,
    WeaponInfo = 21,
    ProtectorInfo = 22,
    AccessoryInfo = 23,
    GoodsCaption = 24,
    WeaponCaption = 25,
    ProtectorCaption = 26,
    AccessoryCaption = 27,
    MagicInfo = 28,
    MagicCaption = 29,
    NetworkMessage = 31,
    ActionButtonText = 32,
    EventTextForTalk = 33,
    EventTextForMap = 34,
    GemName = 35,
    GemInfo = 36,
    GemCaption = 37,
    GoodsDialog = 41,
    ArtsName = 42,
    ArtsCaption = 43,
    WeaponEffect = 44,
    GemEffect = 45,
    GoodsInfo2 = 46,
    GrMenuText = 200,
    GrLineHelp = 201,
    GrKeyGuide = 202,
    GrSystemMessageWin64 = 203,
    GrDialogues = 204,
    LoadingTitle = 205,
    LoadingText = 206,
    TutorialTitle = 207,
    TutorialBody = 208,
    TextEmbedImageNameWin64 = 209,
    ToSWin64 = 210,
    WeaponNameDlc01 = 310,
    WeaponInfoDlc01 = 311,
    WeaponCaptionDlc01 = 312,
    ProtectorNameDlc01 = 313,
    ProtectorInfoDlc01 = 314,
    ProtectorCaptionDlc01 = 315,
    AccessoryNameDlc01 = 316,
    AccessoryInfoDlc01 = 317,
    AccessoryCaptionDlc01 = 318,
    GoodsNameDlc01 = 319,
    GoodsInfoDlc01 = 320,
    GoodsCaptionDlc01 = 321,
    GemNameDlc01 = 322,
    GemInfoDlc01 = 323,
    GemCaptionDlc01 = 324,
    MagicNameDlc01 = 325,
    MagicInfoDlc01 = 326,
    MagicCaptionDlc01 = 327,
    NpcNameDlc01 = 328,
    PlaceNameDlc01 = 329,
    GoodsDialogDlc01 = 330,
    ArtsNameDlc01 = 331,
    ArtsCaptionDlc01 = 332,
    WeaponEffectDlc01 = 333,
    GemEffectDlc01 = 334,
    GoodsInfo2Dlc01 = 335,
    TalkMsgDlc01 = 360,
    BloodMsgDlc01 = 361,
    MovieSubtitleDlc01 = 362,
    NetworkMessageDlc01 = 364,
    ActionButtonTextDlc01 = 365,
    EventTextForTalkDlc01 = 366,
    EventTextForMapDlc01 = 367,
    GrMenuTextDlc01 = 368,
    GrLineHelpDlc01 = 369,
    GrKeyGuideDlc01 = 370,
    GrSystemMessageWin64Dlc01 = 371,
    GrDialoguesDlc01 = 372,
    LoadingTitleDlc01 = 373,
    LoadingTextDlc01 = 374,
    TutorialTitleDlc01 = 375,
    TutorialBodyDlc01 = 376,
    WeaponNameDlc02 = 410,
    WeaponInfoDlc02 = 411,
    WeaponCaptionDlc02 = 412,
    ProtectorNameDlc02 = 413,
    ProtectorInfoDlc02 = 414,
    ProtectorCaptionDlc02 = 415,
    AccessoryNameDlc02 = 416,
    AccessoryInfoDlc02 = 417,
    AccessoryCaptionDlc02 = 418,
    GoodsNameDlc02 = 419,
    GoodsInfoDlc02 = 420,
    GoodsCaptionDlc02 = 421,
    GemNameDlc02 = 422,
    GemInfoDlc02 = 423,
    GemCaptionDlc02 = 424,
    MagicNameDlc02 = 425,
    MagicInfoDlc02 = 426,
    MagicCaptionDlc02 = 427,
    NpcNameDlc02 = 428,
    PlaceNameDlc02 = 429,
    GoodsDialogDlc02 = 430,
    ArtsNameDlc02 = 431,
    ArtsCaptionDlc02 = 432,
    WeaponEffectDlc02 = 433,
    GemEffectDlc02 = 434,
    GoodsInfo2Dlc02 = 435,
    TalkMsgDlc02 = 460,
    BloodMsgDlc02 = 461,
    MovieSubtitleDlc02 = 462,
    NetworkMessageDlc02 = 464,
    ActionButtonTextDlc02 = 465,
    EventTextForTalkDlc02 = 466,
    EventTextForMapDlc02 = 467,
    GrMenuTextDlc02 = 468,
    GrLineHelpDlc02 = 469,
    GrKeyGuideDlc02 = 470,
    GrSystemMessageWin64Dlc02 = 471,
    GrDialoguesDlc02 = 472,
    LoadingTitleDlc02 = 473,
    LoadingTextDlc02 = 474,
    TutorialTitleDlc02 = 475,
    TutorialBodyDlc02 = 476,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct FmgText {
    #[serde(rename = "@id")]
    id: u32,
    #[serde(rename = "#text")]
    content: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct FmgEntries {
    #[serde(rename = "text")]
    texts: Vec<FmgText>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct Fmg {
    filename: String,
    compression: String,
    version: String,
    bigendian: String,
    entries: FmgEntries,
}

#[allow(dead_code)]
struct FmgDatabase {
    entries: Vec<(FmgCategories, Fmg)>,
}

#[allow(dead_code)]
struct FmgDatabases {
    base: FmgDatabase,
    dlc01: Option<FmgDatabase>,
    dlc02: Option<FmgDatabase>,
    custom: Option<FmgDatabase>,
}

#[allow(dead_code)]
impl FromStr for FmgCategories {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TalkMsg.fmg" => Ok(FmgCategories::TalkMsg),
            "BloodMsg.fmg" => Ok(FmgCategories::BloodMsg),
            "MovieSubtitle.fmg" => Ok(FmgCategories::MovieSubtitle),
            "GoodsName.fmg" => Ok(FmgCategories::GoodsName),
            "WeaponName.fmg" => Ok(FmgCategories::WeaponName),
            "ProtectorName.fmg" => Ok(FmgCategories::ProtectorName),
            "AccessoryName.fmg" => Ok(FmgCategories::AccessoryName),
            "MagicName.fmg" => Ok(FmgCategories::MagicName),
            "NpcName.fmg" => Ok(FmgCategories::NpcName),
            "PlaceName.fmg" => Ok(FmgCategories::PlaceName),
            "GoodsInfo.fmg" => Ok(FmgCategories::GoodsInfo),
            "WeaponInfo.fmg" => Ok(FmgCategories::WeaponInfo),
            "ProtectorInfo.fmg" => Ok(FmgCategories::ProtectorInfo),
            "AccessoryInfo.fmg" => Ok(FmgCategories::AccessoryInfo),
            "GoodsCaption.fmg" => Ok(FmgCategories::GoodsCaption),
            "WeaponCaption.fmg" => Ok(FmgCategories::WeaponCaption),
            "ProtectorCaption.fmg" => Ok(FmgCategories::ProtectorCaption),
            "AccessoryCaption.fmg" => Ok(FmgCategories::AccessoryCaption),
            "MagicInfo.fmg" => Ok(FmgCategories::MagicInfo),
            "MagicCaption.fmg" => Ok(FmgCategories::MagicCaption),
            "NetworkMessage.fmg" => Ok(FmgCategories::NetworkMessage),
            "ActionButtonText.fmg" => Ok(FmgCategories::ActionButtonText),
            "EventTextForTalk.fmg" => Ok(FmgCategories::EventTextForTalk),
            "EventTextForMap.fmg" => Ok(FmgCategories::EventTextForMap),
            "GemName.fmg" => Ok(FmgCategories::GemName),
            "GemInfo.fmg" => Ok(FmgCategories::GemInfo),
            "GemCaption.fmg" => Ok(FmgCategories::GemCaption),
            "GoodsDialog.fmg" => Ok(FmgCategories::GoodsDialog),
            "ArtsName.fmg" => Ok(FmgCategories::ArtsName),
            "ArtsCaption.fmg" => Ok(FmgCategories::ArtsCaption),
            "WeaponEffect.fmg" => Ok(FmgCategories::WeaponEffect),
            "GemEffect.fmg" => Ok(FmgCategories::GemEffect),
            "GoodsInfo2.fmg" => Ok(FmgCategories::GoodsInfo2),
            "GrMenuText.fmg" => Ok(FmgCategories::GrMenuText),
            "GrLineHelp.fmg" => Ok(FmgCategories::GrLineHelp),
            "GrKeyGuide.fmg" => Ok(FmgCategories::GrKeyGuide),
            "GrSystemMessageWin64.fmg" => {
                Ok(FmgCategories::GrSystemMessageWin64)
            }
            "GrDialogues.fmg" => Ok(FmgCategories::GrDialogues),
            "LoadingTitle.fmg" => Ok(FmgCategories::LoadingTitle),
            "LoadingText.fmg" => Ok(FmgCategories::LoadingText),
            "TutorialTitle.fmg" => Ok(FmgCategories::TutorialTitle),
            "TutorialBody.fmg" => Ok(FmgCategories::TutorialBody),
            "TextEmbedImageNameWin64.fmg" => {
                Ok(FmgCategories::TextEmbedImageNameWin64)
            }
            "ToSWin64.fmg" => Ok(FmgCategories::ToSWin64),
            "WeaponNameDlc01.fmg" => Ok(FmgCategories::WeaponNameDlc01),
            "WeaponInfoDlc01.fmg" => Ok(FmgCategories::WeaponInfoDlc01),
            "WeaponCaptionDlc01.fmg" => Ok(FmgCategories::WeaponCaptionDlc01),
            "ProtectorNameDlc01.fmg" => Ok(FmgCategories::ProtectorNameDlc01),
            "ProtectorInfoDlc01.fmg" => Ok(FmgCategories::ProtectorInfoDlc01),
            "ProtectorCaptionDlc01.fmg" => {
                Ok(FmgCategories::ProtectorCaptionDlc01)
            }
            "AccessoryNameDlc01.fmg" => Ok(FmgCategories::AccessoryNameDlc01),
            "AccessoryInfoDlc01.fmg" => Ok(FmgCategories::AccessoryInfoDlc01),
            "AccessoryCaptionDlc01.fmg" => {
                Ok(FmgCategories::AccessoryCaptionDlc01)
            }
            "GoodsNameDlc01.fmg" => Ok(FmgCategories::GoodsNameDlc01),
            "GoodsInfoDlc01.fmg" => Ok(FmgCategories::GoodsInfoDlc01),
            "GoodsCaptionDlc01.fmg" => Ok(FmgCategories::GoodsCaptionDlc01),
            "GemNameDlc01.fmg" => Ok(FmgCategories::GemNameDlc01),
            "GemInfoDlc01.fmg" => Ok(FmgCategories::GemInfoDlc01),
            "GemCaptionDlc01.fmg" => Ok(FmgCategories::GemCaptionDlc01),
            "MagicNameDlc01.fmg" => Ok(FmgCategories::MagicNameDlc01),
            "MagicInfoDlc01.fmg" => Ok(FmgCategories::MagicInfoDlc01),
            "MagicCaptionDlc01.fmg" => Ok(FmgCategories::MagicCaptionDlc01),
            "NpcNameDlc01.fmg" => Ok(FmgCategories::NpcNameDlc01),
            "PlaceNameDlc01.fmg" => Ok(FmgCategories::PlaceNameDlc01),
            "GoodsDialogDlc01.fmg" => Ok(FmgCategories::GoodsDialogDlc01),
            "ArtsNameDlc01.fmg" => Ok(FmgCategories::ArtsNameDlc01),
            "ArtsCaptionDlc01.fmg" => Ok(FmgCategories::ArtsCaptionDlc01),
            "WeaponEffectDlc01.fmg" => Ok(FmgCategories::WeaponEffectDlc01),
            "GemEffectDlc01.fmg" => Ok(FmgCategories::GemEffectDlc01),
            "GoodsInfo2Dlc01.fmg" => Ok(FmgCategories::GoodsInfo2Dlc01),
            "TalkMsgDlc01.fmg" => Ok(FmgCategories::TalkMsgDlc01),
            "BloodMsgDlc01.fmg" => Ok(FmgCategories::BloodMsgDlc01),
            "MovieSubtitleDlc01.fmg" => Ok(FmgCategories::MovieSubtitleDlc01),
            "NetworkMessageDlc01.fmg" => Ok(FmgCategories::NetworkMessageDlc01),
            "ActionButtonTextDlc01.fmg" => {
                Ok(FmgCategories::ActionButtonTextDlc01)
            }
            "EventTextForTalkDlc01.fmg" => {
                Ok(FmgCategories::EventTextForTalkDlc01)
            }
            "EventTextForMapDlc01.fmg" => {
                Ok(FmgCategories::EventTextForMapDlc01)
            }
            "GrMenuTextDlc01.fmg" => Ok(FmgCategories::GrMenuTextDlc01),
            "GrLineHelpDlc01.fmg" => Ok(FmgCategories::GrLineHelpDlc01),
            "GrKeyGuideDlc01.fmg" => Ok(FmgCategories::GrKeyGuideDlc01),
            "GrSystemMessageWin64Dlc01.fmg" => {
                Ok(FmgCategories::GrSystemMessageWin64Dlc01)
            }
            "GrDialoguesDlc01.fmg" => Ok(FmgCategories::GrDialoguesDlc01),
            "LoadingTitleDlc01.fmg" => Ok(FmgCategories::LoadingTitleDlc01),
            "LoadingTextDlc01.fmg" => Ok(FmgCategories::LoadingTextDlc01),
            "TutorialTitleDlc01.fmg" => Ok(FmgCategories::TutorialTitleDlc01),
            "TutorialBodyDlc01.fmg" => Ok(FmgCategories::TutorialBodyDlc01),
            "WeaponNameDlc02.fmg" => Ok(FmgCategories::WeaponNameDlc02),
            "WeaponInfoDlc02.fmg" => Ok(FmgCategories::WeaponInfoDlc02),
            "WeaponCaptionDlc02.fmg" => Ok(FmgCategories::WeaponCaptionDlc02),
            "ProtectorNameDlc02.fmg" => Ok(FmgCategories::ProtectorNameDlc02),
            "ProtectorInfoDlc02.fmg" => Ok(FmgCategories::ProtectorInfoDlc02),
            "ProtectorCaptionDlc02.fmg" => {
                Ok(FmgCategories::ProtectorCaptionDlc02)
            }
            "AccessoryNameDlc02.fmg" => Ok(FmgCategories::AccessoryNameDlc02),
            "AccessoryInfoDlc02.fmg" => Ok(FmgCategories::AccessoryInfoDlc02),
            "AccessoryCaptionDlc02.fmg" => {
                Ok(FmgCategories::AccessoryCaptionDlc02)
            }
            "GoodsNameDlc02.fmg" => Ok(FmgCategories::GoodsNameDlc02),
            "GoodsInfoDlc02.fmg" => Ok(FmgCategories::GoodsInfoDlc02),
            "GoodsCaptionDlc02.fmg" => Ok(FmgCategories::GoodsCaptionDlc02),
            "GemNameDlc02.fmg" => Ok(FmgCategories::GemNameDlc02),
            "GemInfoDlc02.fmg" => Ok(FmgCategories::GemInfoDlc02),
            "GemCaptionDlc02.fmg" => Ok(FmgCategories::GemCaptionDlc02),
            "MagicNameDlc02.fmg" => Ok(FmgCategories::MagicNameDlc02),
            "MagicInfoDlc02.fmg" => Ok(FmgCategories::MagicInfoDlc02),
            "MagicCaptionDlc02.fmg" => Ok(FmgCategories::MagicCaptionDlc02),
            "NpcNameDlc02.fmg" => Ok(FmgCategories::NpcNameDlc02),
            "PlaceNameDlc02.fmg" => Ok(FmgCategories::PlaceNameDlc02),
            "GoodsDialogDlc02.fmg" => Ok(FmgCategories::GoodsDialogDlc02),
            "ArtsNameDlc02.fmg" => Ok(FmgCategories::ArtsNameDlc02),
            "ArtsCaptionDlc02.fmg" => Ok(FmgCategories::ArtsCaptionDlc02),
            "WeaponEffectDlc02.fmg" => Ok(FmgCategories::WeaponEffectDlc02),
            "GemEffectDlc02.fmg" => Ok(FmgCategories::GemEffectDlc02),
            "GoodsInfo2Dlc02.fmg" => Ok(FmgCategories::GoodsInfo2Dlc02),
            "TalkMsgDlc02.fmg" => Ok(FmgCategories::TalkMsgDlc02),
            "BloodMsgDlc02.fmg" => Ok(FmgCategories::BloodMsgDlc02),
            "MovieSubtitleDlc02.fmg" => Ok(FmgCategories::MovieSubtitleDlc02),
            "NetworkMessageDlc02.fmg" => Ok(FmgCategories::NetworkMessageDlc02),
            "ActionButtonTextDlc02.fmg" => {
                Ok(FmgCategories::ActionButtonTextDlc02)
            }
            "EventTextForTalkDlc02.fmg" => {
                Ok(FmgCategories::EventTextForTalkDlc02)
            }
            "EventTextForMapDlc02.fmg" => {
                Ok(FmgCategories::EventTextForMapDlc02)
            }
            "GrMenuTextDlc02.fmg" => Ok(FmgCategories::GrMenuTextDlc02),
            "GrLineHelpDlc02.fmg" => Ok(FmgCategories::GrLineHelpDlc02),
            "GrKeyGuideDlc02.fmg" => Ok(FmgCategories::GrKeyGuideDlc02),
            "GrSystemMessageWin64Dlc02.fmg" => {
                Ok(FmgCategories::GrSystemMessageWin64Dlc02)
            }
            "GrDialoguesDlc02.fmg" => Ok(FmgCategories::GrDialoguesDlc02),
            "LoadingTitleDlc02.fmg" => Ok(FmgCategories::LoadingTitleDlc02),
            "LoadingTextDlc02.fmg" => Ok(FmgCategories::LoadingTextDlc02),
            "TutorialTitleDlc02.fmg" => Ok(FmgCategories::TutorialTitleDlc02),
            "TutorialBodyDlc02.fmg" => Ok(FmgCategories::TutorialBodyDlc02),
            _ => Err(()),
        }
    }
}

#[allow(dead_code)]
impl Fmg {
    pub fn get_entry(&self, id: u32) -> Option<&FmgText> {
        self.entries.texts.iter().find(|text| text.id == id)
    }
}

#[allow(dead_code)]
impl FmgDatabases {
    pub fn new() -> Self {
        Self {
            base: FmgDatabase {
                entries: vec![],
            },
            dlc01: None,
            dlc02: None,
            custom: None,
        }
    }

    pub fn get_fmg(&self, category: FmgCategories) -> Option<&Fmg> {
        self.base.entries.iter().find_map(|(cat, fmg)| {
            if *cat == category {
                Some(fmg)
            } else {
                None
            }
        })
    }

    pub fn read_from_dirs(
        &mut self,
        additional_custom_path: Option<&str>,
    ) -> &mut Self {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"));
        let path = path.parent().unwrap().parent().unwrap();
        let path = path.join("resources\\");
        let path = path.as_path();
        let path_suffixes = [
            "item-msgbnd-dcx",
            "item_dlc01-msgbnd-dcx",
            "item_dlc02-msgbnd-dcx",
        ];
        if path.exists() {
            self.base =
                self.read_fmg_files(path, Some(path_suffixes[0])).unwrap();
            self.dlc01 = self.read_fmg_files(path, Some(path_suffixes[1]));
            self.dlc02 = self.read_fmg_files(path, Some(path_suffixes[2]));
            self.custom = self.read_fmg_files(path, additional_custom_path);
        }

        self
    }

    pub fn read_fmg_files(
        &mut self,
        base_path: &Path,
        path_suffix: Option<&str>,
    ) -> Option<FmgDatabase> {
        path_suffix?;
        let path_suffix = path_suffix.unwrap();
        let full_path = base_path.join(path_suffix);
        if !full_path.exists() {
            return None;
        }
        let mut database = FmgDatabase {
            entries: vec![],
        };
        // Iterate through each .xml file in the directory
        if let Ok(entries) = std::fs::read_dir(&full_path) {
            for entry in entries.flatten() {
                if entry.path().extension().and_then(|s| s.to_str())
                    == Some("xml")
                {
                    if let Ok(xml_content) =
                        std::fs::read_to_string(entry.path())
                    {
                        if let Ok(fmg) =
                            serde_xml_rs::from_str::<Fmg>(&xml_content)
                        {
                            // Use the filename to determine the category
                            let category = fmg
                                .filename
                                .parse::<FmgCategories>()
                                .unwrap_or(FmgCategories::GoodsName);
                            database.entries.push((category, fmg));
                        }
                    }
                }
            }
        }

        Some(database)
    }

    pub fn add_fmg(&mut self, fmg: Fmg, category: FmgCategories) {
        if !self.base.entries.contains(&(category, fmg.clone())) {
            self.base.entries.push((category, fmg));
        }
    }
}

#[cfg(test)]
mod test {
    use crate::fmg::Fmg;
    use crate::fmg::FmgCategories;
    use crate::fmg::FmgDatabases;
    use crate::fmg::FmgEntries;
    use crate::fmg::FmgText;
    use crate::fmg::Path;
    use serde_xml_rs::from_str;
    #[test]
    fn test_get_fmg_entry() {
        let fmg = Fmg {
            filename: "Test.fmg".to_string(),
            compression: "none".to_string(),
            version: "1.0".to_string(),
            bigendian: "false".to_string(),
            entries: FmgEntries {
                texts: vec![
                    FmgText {
                        id: 1,
                        content: "Test content 1".to_string(),
                    },
                    FmgText {
                        id: 2,
                        content: "Test content 2".to_string(),
                    },
                ],
            },
        };
        assert_eq!(fmg.get_entry(1).unwrap().content, "Test content 1");
        assert!(fmg.get_entry(3).is_none(), "Entry with id 3 should not exist");
    }

    #[test]
    fn test_get_fmg() {
        let mut db = FmgDatabases::new();
        db.read_from_dirs(None);
        let fmg = db.get_fmg(FmgCategories::GoodsName);
        assert!(fmg.is_some(), "FMG for GoodsName should be found");
        assert_eq!(
            fmg.unwrap().filename,
            "GoodsName.fmg",
            "FMG filename should match"
        );
    }

    #[test]
    fn test_add_fmg() {
        let mut db = FmgDatabases::new();
        let fmg = Fmg {
            filename: "Test.fmg".to_string(),
            compression: "none".to_string(),
            version: "1.0".to_string(),
            bigendian: "false".to_string(),
            entries: FmgEntries {
                texts: vec![FmgText {
                    id: 1,
                    content: "Test content".to_string(),
                }],
            },
        };
        db.add_fmg(fmg.clone(), FmgCategories::GoodsName);
        assert!(
            !db.base.entries.is_empty(),
            "Base FMG entries should not be empty"
        );
        assert!(
            db.base.entries.contains(&(FmgCategories::GoodsName, fmg.clone())),
            "FMG should be added to the base database"
        );
        db.add_fmg(fmg.clone(), FmgCategories::GoodsName);
        assert_eq!(
            db.base.entries.len(),
            1,
            "FMG should not be duplicated in the base database"
        );
    }

    #[test]
    fn test_serialize() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"));
        let path = path.parent().unwrap().parent().unwrap();
        let path = path.join("resources\\item-msgbnd-dcx\\ArtsCaption.fmg.xml");

        let xml_content =
            std::fs::read_to_string(path).expect("Failed to read XML file");
        let fmg: Fmg = from_str(&xml_content).expect("Failed to parse XML");
        assert_eq!(fmg.filename, "ArtsCaption.fmg");
    }

    #[test]
    fn test_read_from_dirs() {
        let mut db = FmgDatabases::new();
        db.read_from_dirs(None);
        assert!(
            !db.base.entries.is_empty(),
            "Base FMG entries should not be empty"
        );
        if let Some(dlc01) = &db.dlc01 {
            assert!(
                !dlc01.entries.is_empty(),
                "DLC01 FMG entries should not be empty"
            );
        }
        if let Some(dlc02) = &db.dlc02 {
            assert!(
                !dlc02.entries.is_empty(),
                "DLC02 FMG entries should not be empty"
            );
        }
        assert!(db.custom.is_none(), "Custom FMG entries should be None");
        db.read_from_dirs(Some("item_custom-msgbnd-dcx"));
        if let Some(custom) = &db.custom {
            assert!(
                !custom.entries.is_empty(),
                "Custom FMG entries should not be empty"
            );
        } else {
            panic!(
                "Custom FMG entries should not be None after reading from custom path"
            );
        }
    }
}
