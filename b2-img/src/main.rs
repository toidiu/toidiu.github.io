use itertools::Itertools;
use reqwest::header;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

const CHUNK_SIZE: usize = 10;
const FILE_NAME: &str = "europe-2019";
// {
//   "files": [
//     {
//       "accountId": "c2dd060c8902",
//       "action": "upload",
//       "bucketId": "dc423ded3016303c68c90012",
//       "contentLength": 1600802,
//       "contentSha1": "d5117aeca952929f4d6300844860505f7c931d6c",
//       "contentType": "image/jpeg",
//       "fileId": "4_zdc423ded3016303c68c90012_f1109b5a2d761bd7e_d20190727_m124745_c002_v0001107_t0046",
//       "fileInfo": {
//         "src_last_modified_millis": "1564230586000"
//       },
//       "fileName": "IMG_20190429_142313.jpg",
//       "uploadTimestamp": 1564231665000
//     },
//     ...
//   ]
// }
// https://f002.backblazeb2.com/file/europe-2019/IMG_20190429_142313.jpg

#[derive(Serialize, Deserialize, Debug)]
struct B2FileList {
    files: Vec<B2File>,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct B2File {
    fileName: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct ActualFileUrl {
    path: String,
}
impl From<B2File> for ActualFileUrl {
    fn from(item: B2File) -> Self {
        ActualFileUrl {
            path: format!(
                "https://photos.toidiu.com/file/europe-2019/{}",
                item.fileName
            ),
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct MdFile {
    content: String,
}
impl MdFile {
    fn new(idx: usize, items: Vec<ActualFileUrl>) -> Self {
        let mut s = String::new();

        s.push_str(&format!(
            "+++
title = \"{}-{}\"
date = 2019-07-01

[taxonomies]
tag = [\"travel\"]

[extra]
id = photos-single
+++\n\n",
            FILE_NAME, idx
        ));

        for url in items {
            s.push_str("<div class='pixels-photo is-large'>\n");
            s.push_str(&format!("  <img src='{}' alt='img'>\n", url.path));
            s.push_str("</div>\n");
            s.push_str("<br/>\n\n");
        }
        MdFile { content: s }
    }
}

fn main() {
    // curl -H "Authorization: $ACCOUNT_AUTH_TOKEN" -d "{\"bucketId\": \"$BUCKET_ID\"}" https://api002.backblazeb2.com/b2api/v2/b2_list_file_names
    let token = std::env::var("ACCOUNT_AUTH_TOKEN").expect("no 'ACCOUNT_AUTH_TOKEN' in env");
    let bucket_id = std::env::var("BUCKET_ID").expect("no 'BUCKET_ID' in env");

    let mut map = HashMap::new();
    map.insert("bucketId", bucket_id);
    let client = reqwest::Client::builder().build().unwrap();
    let mut res = client
        .post("https://api002.backblazeb2.com/b2api/v2/b2_list_file_names")
        .header(header::AUTHORIZATION, token)
        .json(&map)
        .send()
        .unwrap();
    let list: B2FileList = res.json().unwrap();

    //---------
    let files: Vec<ActualFileUrl> = list.files.into_iter().map(|x: B2File| x.into()).collect();
    let mut chunked_urls: Vec<Vec<ActualFileUrl>> = Vec::new();
    for chunk in &files.into_iter().chunks(CHUNK_SIZE) {
        let b: Vec<ActualFileUrl> = chunk.into_iter().collect();
        chunked_urls.push(b);
    }

    generate_html(chunked_urls);
}

fn generate_html(chunked_urls: Vec<Vec<ActualFileUrl>>) {
    let _md_files: Vec<()> = chunked_urls
        .into_iter()
        .enumerate()
        .map(|x| {
            let idx = x.0 + 1;
            let vec_actual_file_url = x.1;
            let md_file = MdFile::new(idx, vec_actual_file_url);
            let file_name = format!("../content/photos/{}-{}.md", FILE_NAME, idx);
            let mut file = File::create(file_name).expect("unable to create file");
            file.write_all(md_file.content.as_bytes())
                .expect("unable to write file");
        })
        .collect();
    // for f in md_files {
    //     println!("{}", f.content);
    //     let mut file = File::create("img-.md").expect("unable to create file");
    //     file.write_all()
    //         .expect("unable to write file");
    // }
}
