use crate::utils::{ Link };

pub struct AndroidAppData {
    pub title: String,
    pub description: String,
    pub download_links: Vec<Link>,
    pub image_filename: String,
    pub privay_url: String
}

/**
 * We won't have any links until we publish the games.
 */
pub fn get_android_apps_data() -> Vec<AndroidAppData> {

    // CRANKWORD DATA

    let crankword_links: Vec<Link> = vec![
        Link {
            url: "#".to_string(),
            title: "COMING SOON!".to_string(),
        },
    ];

    let crankword_privacy_url: String = "/static/crankword_privacy_policy.html".to_string();

    let crawnkword_data: AndroidAppData = AndroidAppData {
        title: "Crankword".to_string(),
        description: "A word-guessing game.".to_string(),
        download_links: crankword_links,
        image_filename: "crankword_icon.png".to_string(),
        privay_url: crankword_privacy_url,
    };


    // CRIBBAGE MATRIX DATA


    let cribbage_matrix_links: Vec<Link> = vec![
        Link {
            url: "#".to_string(),
            title: "COMING SOON!".to_string(),
        },
    ];

    let cribbage_matrix_privacy_url: String = "/static/cribbage_matrix_privacy_policy.html".to_string();

    let cribbage_matrix_data: AndroidAppData = AndroidAppData {
        title: "Cribbage Matrix".to_string(),
        description: "Use Cribbage rules to score points with combinations on the board.".to_string(),
        download_links: cribbage_matrix_links,
        image_filename: "cribbage_matrix_icon.png".to_string(),
        privay_url: cribbage_matrix_privacy_url,
    };

    vec![
        crawnkword_data,
        cribbage_matrix_data
    ]
}