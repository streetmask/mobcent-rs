use crate::board::Board;
use crate::utils::*;

impl UrlWithQuery<ForumList> {
    pub fn new() -> Result<Self, Error> {
        Ok(Self(
            Url::from_str("https://bbs.uestc.edu.cn/mobcent/app/web/index.php?r=forum/forumlist")?,
            PhantomData,
        ))
    }

    pub fn fid(mut self, fid: Id) -> Self {
        self.0
            .query_pairs_mut()
            .append_pair("fid", fid.to_string().as_str());
        dbg!(&self.0);
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ForumList {
    #[serde(flatten)]
    common_header: CommonHeader,
    list: Vec<BoardCategory>,
    online_user_num: Number,
    td_visitors: Number,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct BoardCategory {
    board_category_id: Number,
    board_category_name: String,
    board_category_type: Id,
    board_list: Vec<Board>,
}

impl ForumList {
    pub fn new() -> Result<UrlWithQuery<ForumList>, Error> {
        Ok(UrlWithQuery::<ForumList>::new()?)
    }
}

// #[test]
// fn handcode() {
//     let fl = ForumList {
//         common_header: CommonHeader {
//             rs: Boolean::True,
//             err_code: "".to_owned(),
//             head: Head {
//                 err_code: "00000000".to_owned(),
//                 err_info: "....".to_owned(),
//                 version: "2.6.1.7".to_owned(),
//                 alert: 0,
//             },
//             body: Body {
//                 extern_info: ExternInfo {
//                     padding: "".to_owned(),
//                 },
//             },
//         },
//         list: vec![BoardCategory {
//             board_category_id: 1,
//             board_category_name: "综合".to_owned(),
//             board_category_type: 221,
//             board_list: vec![Board {
//                 board_id: 1,
//                 board_name: "跑步".to_owned(),
//                 description: "".to_owned(),
//                 board_child: 1,
//                 board_img: "".to_owned(),
//                 last_posts_date: "".to_owned(),
//                 board_content: 2,
//                 forum_redirect: "".to_owned(),
//                 fav_num: 2,
//                 td_posts_num: 200,
//                 topic_total_num: 20123,
//                 posts_total_num: 324341,
//                 is_focus: Boolean::True,
//             }],
//         }],
//         online_user_num: 1000,
//         td_visitors: 312412,
//     };

//     if let Ok(ser) = serde_json::to_string(&fl) {
//         if let Ok(js) = json::parse(&ser.as_str()) {
//             println!("{:#}", js);
//         }

//         if let Ok(de) = serde_json::from_str::<ForumList>(ser.as_str()) {
//             dbg!(de);
//         }
//     }
// }
