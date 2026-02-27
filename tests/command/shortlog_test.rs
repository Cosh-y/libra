
use clap::Parser;
use git_internal::{
    hash::ObjectHash,
    internal::object::{
        commit::Commit,
        signature::{Signature, SignatureType},
    },
};
use libra::internal::log::date_parser::parse_date;

use super::*;

fn create_signature(signature_type: SignatureType, name: &str) -> Signature {
    Signature::from_data(
        format!(
            "{} {} <{}@oa.org> {} +0800",
            match signature_type {
                SignatureType::Author => "author",
                SignatureType::Committer => "committer",
                _ => panic!("Unsupported signature type"),
            },
            name,
            name.to_lowercase(),
            chrono::Utc::now().timestamp()
        )
        .to_string()
        .into_bytes(),
    )
    .unwrap()
}

/// create a test commit tree structure as graph and create branch (master) head to commit 6
/// return a commit hash of commit 6
///             3(SHY) --  6(SHY)
///            /          /
///    1(LEAVE)  --  4(SHY)  --  5(SHY) -- 7(GUXUE) -- 10(MMONK) -- 14(SunZo)
///            \     /                                 /            /  
///             2(LEAVE)  --  8(LENGSA)  --  9(SunZo)              /
///              \                                                /
///               11(LEAVE) -- 12(LEAVE) -- 13(SHY) ---- ---- ---
/// The time of commit and the commit number should be in the same order.
async fn create_test_commit_tree() -> String {
    let mut commit_1 = Commit::new(
        create_signature(SignatureType::Author, "LEAVE"),
        create_signature(SignatureType::Committer, "LEAVE"),
        ObjectHash::new(&[1; 20]),
        vec![],
        &format_commit_msg("Commit_1", None),
    );
    commit_1.author.timestamp = parse_date("2026-01-01").unwrap() as usize;
    save_object(&commit_1, &commit_1.id).unwrap();

    let mut commit_2 = Commit::new(
        create_signature(SignatureType::Author, "LEAVE"),
        create_signature(SignatureType::Committer, "LEAVE"),
        ObjectHash::new(&[2; 20]),
        vec![commit_1.id],
        &format_commit_msg("Commit_2", None),
    );
    commit_2.author.timestamp = parse_date("2026-01-02").unwrap() as usize;
    save_object(&commit_2, &commit_2.id).unwrap();

    let mut commit_3 = Commit::new(
        create_signature(SignatureType::Author, "SHY"),
        create_signature(SignatureType::Committer, "SHY"),
        ObjectHash::new(&[3; 20]),
        vec![commit_1.id],
        &format_commit_msg("Commit_3", None),
    );
    commit_3.author.timestamp = parse_date("2026-01-03").unwrap() as usize;
    save_object(&commit_3, &commit_3.id).unwrap();

    let mut commit_4 = Commit::new(
        create_signature(SignatureType::Author, "LEAVE"),
        create_signature(SignatureType::Committer, "LEAVE"),
        ObjectHash::new(&[4; 20]),
        vec![commit_1.id, commit_2.id],
        &format_commit_msg("Commit_4", None),
    );
    commit_4.author.timestamp = parse_date("2026-01-04").unwrap() as usize;
    save_object(&commit_4, &commit_4.id).unwrap();

    let mut commit_5 = Commit::new(
        create_signature(SignatureType::Author, "SHY"),
        create_signature(SignatureType::Committer, "SHY"),
        ObjectHash::new(&[5; 20]),
        vec![commit_4.id],
        &format_commit_msg("Commit_5", None),
    );
    commit_5.author.timestamp = parse_date("2026-01-05").unwrap() as usize;
    save_object(&commit_5, &commit_5.id).unwrap();

    let mut commit_6 = Commit::new(
        create_signature(SignatureType::Author, "SHY"),
        create_signature(SignatureType::Committer, "SHY"),
        ObjectHash::new(&[6; 20]),
        vec![commit_3.id, commit_4.id],
        &format_commit_msg("Commit_6", None),
    );
    commit_6.author.timestamp = parse_date("2026-01-06").unwrap() as usize;
    save_object(&commit_6, &commit_6.id).unwrap();

    let mut commit_7 = Commit::new(
        create_signature(SignatureType::Author, "GUXUE"),
        create_signature(SignatureType::Committer, "GUXUE"),
        ObjectHash::new(&[7; 20]),
        vec![commit_5.id],
        &format_commit_msg("Commit_7", None),
    );
    commit_7.author.timestamp = parse_date("2026-01-07").unwrap() as usize;
    save_object(&commit_7, &commit_7.id).unwrap();

    let mut commit_8 = Commit::new(
        create_signature(SignatureType::Author, "LENGSA"),
        create_signature(SignatureType::Committer, "LENGSA"),
        ObjectHash::new(&[8; 20]),
        vec![commit_2.id],
        &format_commit_msg("Commit_8", None),
    );
    commit_8.author.timestamp = parse_date("2026-01-08").unwrap() as usize;
    save_object(&commit_8, &commit_8.id).unwrap();

    let mut commit_9 = Commit::new(
        create_signature(SignatureType::Author, "SunZo"),
        create_signature(SignatureType::Committer, "SunZo"),
        ObjectHash::new(&[9; 20]),
        vec![commit_8.id],
        &format_commit_msg("Commit_9", None),
    );
    commit_9.author.timestamp = parse_date("2026-01-09").unwrap() as usize;
    save_object(&commit_9, &commit_9.id).unwrap();

    let mut commit_10 = Commit::new(
        create_signature(SignatureType::Author, "MMONK"),
        create_signature(SignatureType::Committer, "MMONK"),
        ObjectHash::new(&[10; 20]),
        vec![commit_7.id, commit_9.id],
        &format_commit_msg("Commit_10", None),
    );
    commit_10.author.timestamp = parse_date("2026-01-10").unwrap() as usize;
    save_object(&commit_10, &commit_10.id).unwrap();

    let mut commit_11 = Commit::new(
        create_signature(SignatureType::Author, "LEAVE"),
        create_signature(SignatureType::Committer, "LEAVE"),
        ObjectHash::new(&[11; 20]),
        vec![commit_2.id],
        &format_commit_msg("Commit_11", None),
    );
    commit_11.author.timestamp = parse_date("2026-01-11").unwrap() as usize;
    save_object(&commit_11, &commit_11.id).unwrap();

    let mut commit_12 = Commit::new(
        create_signature(SignatureType::Author, "LEAVE"),
        create_signature(SignatureType::Committer, "LEAVE"),
        ObjectHash::new(&[12; 20]),
        vec![commit_11.id],
        &format_commit_msg("Commit_12", None),
    );
    commit_12.author.timestamp = parse_date("2026-01-12").unwrap() as usize;
    save_object(&commit_12, &commit_12.id).unwrap();

    let mut commit_13 = Commit::new(
        create_signature(SignatureType::Author, "SHY"),
        create_signature(SignatureType::Committer, "SHY"),
        ObjectHash::new(&[13; 20]),
        vec![commit_12.id],
        &format_commit_msg("Commit_13", None),
    );
    commit_13.author.timestamp = parse_date("2026-01-13").unwrap() as usize;
    save_object(&commit_13, &commit_13.id).unwrap();

    let mut commit_14 = Commit::new(
        create_signature(SignatureType::Author, "SunZo"),
        create_signature(SignatureType::Committer, "SunZo"),
        ObjectHash::new(&[14; 20]),
        vec![commit_10.id, commit_13.id],
        &format_commit_msg("Commit_14", None),
    );
    commit_14.author.timestamp = parse_date("2026-01-14").unwrap() as usize;
    save_object(&commit_14, &commit_14.id).unwrap();

    // set current branch head to commit 14
    let head = Head::current().await;
    let branch_name = match head {
        Head::Branch(name) => name,
        _ => panic!("should be branch"),
    };

    Branch::update_branch(&branch_name, &commit_14.id.to_string(), None).await;

    commit_14.id.to_string()
}

#[tokio::test]
#[serial]
async fn test_execute_shortlog() {
    let temp_path = tempdir().unwrap();
    test::setup_with_new_libra_in(temp_path.path()).await;
    let _guard = ChangeDirGuard::new(temp_path.path());
    let _ = create_test_commit_tree().await;

    // test shortlog command without options
    let args = ShortlogArgs::try_parse_from(["libra"]).unwrap();
    // shortlog::execute_to(args, &mut std::io::stdout()).await;

    let mut buf = Vec::new();
    shortlog::execute_to(args, &mut buf).await;
    let output = String::from_utf8(buf).unwrap();

    // expected output
    let expected = r#"   1  GUXUE
      Commit_7
   5  LEAVE
      Commit_12
      Commit_11
      Commit_4
      Commit_2
      Commit_1
   1  LENGSA
      Commit_8
   1  MMONK
      Commit_10
   2  SHY
      Commit_13
      Commit_5
   2  SunZo
      Commit_14
      Commit_9
"#;

    let out_lines: Vec<_> = output.lines().collect();
    let exp_lines: Vec<_> = expected.lines().collect();
    assert_eq!(out_lines, exp_lines);

    // test shortlog command with -n option
    let args = ShortlogArgs::try_parse_from(["libra", "-n"]).unwrap();
    // shortlog::execute_to(args, &mut std::io::stdout()).await;
    let mut buf = Vec::new();
    shortlog::execute_to(args, &mut buf).await;
    let output = String::from_utf8(buf).unwrap();

    let expected = r#"   5  LEAVE
      Commit_12
      Commit_11
      Commit_4
      Commit_2
      Commit_1
   2  SHY
      Commit_13
      Commit_5
   2  SunZo
      Commit_14
      Commit_9
   1  GUXUE
      Commit_7
   1  LENGSA
      Commit_8
   1  MMONK
      Commit_10
"#;

    let out_lines: Vec<_> = output.lines().collect();
    let exp_lines: Vec<_> = expected.lines().collect();
    assert_eq!(out_lines, exp_lines);

    // test shortlog command with -s option
    let args = ShortlogArgs::try_parse_from(["libra", "-s"]).unwrap();
    // shortlog::execute_to(args, &mut std::io::stdout()).await;
    let mut buf = Vec::new();
    shortlog::execute_to(args, &mut buf).await;
    let output = String::from_utf8(buf).unwrap();

    let expected = r#"   1  GUXUE
   5  LEAVE
   1  LENGSA
   1  MMONK
   2  SHY
   2  SunZo
"#;

    let out_lines: Vec<_> = output.lines().collect();
    let exp_lines: Vec<_> = expected.lines().collect();
    assert_eq!(out_lines, exp_lines);

    // test shortlog command with -e option
    let args = ShortlogArgs::try_parse_from(["libra", "-e"]).unwrap();
    // shortlog::execute_to(args, &mut std::io::stdout()).await;
    let mut buf = Vec::new();
    shortlog::execute_to(args, &mut buf).await;
    let output = String::from_utf8(buf).unwrap();

    let expected = r#"   1  GUXUE <guxue@oa.org>
      Commit_7
   5  LEAVE <leave@oa.org>
      Commit_12
      Commit_11
      Commit_4
      Commit_2
      Commit_1
   1  LENGSA <lengsa@oa.org>
      Commit_8
   1  MMONK <mmonk@oa.org>
      Commit_10
   2  SHY <shy@oa.org>
      Commit_13
      Commit_5
   2  SunZo <sunzo@oa.org>
      Commit_14
      Commit_9
"#;

    let out_lines: Vec<_> = output.lines().collect();
    let exp_lines: Vec<_> = expected.lines().collect();
    assert_eq!(out_lines, exp_lines);

    // test shortlog command with -n -s options
    let args = ShortlogArgs::try_parse_from(["libra", "-n", "-s"]).unwrap();
    // shortlog::execute_to(args, &mut std::io::stdout()).await;
    let mut buf = Vec::new();
    shortlog::execute_to(args, &mut buf).await;
    let output = String::from_utf8(buf).unwrap();

    let expected = r#"   5  LEAVE
   2  SHY
   2  SunZo
   1  GUXUE
   1  LENGSA
   1  MMONK
"#;

    let out_lines: Vec<_> = output.lines().collect();
    let exp_lines: Vec<_> = expected.lines().collect();
    assert_eq!(out_lines, exp_lines);

    // test shortlog command with -n -e options
    let args = ShortlogArgs::try_parse_from(["libra", "-n", "-e"]).unwrap();
    // shortlog::execute_to(args, &mut std::io::stdout()).await;
    let mut buf = Vec::new();
    shortlog::execute_to(args, &mut buf).await;
    let output = String::from_utf8(buf).unwrap();

    let expected = r#"   5  LEAVE <leave@oa.org>
      Commit_12
      Commit_11
      Commit_4
      Commit_2
      Commit_1
   2  SHY <shy@oa.org>
      Commit_13
      Commit_5
   2  SunZo <sunzo@oa.org>
      Commit_14
      Commit_9
   1  GUXUE <guxue@oa.org>
      Commit_7
   1  LENGSA <lengsa@oa.org>
      Commit_8
   1  MMONK <mmonk@oa.org>
      Commit_10
"#;

    let out_lines: Vec<_> = output.lines().collect();
    let exp_lines: Vec<_> = expected.lines().collect();
    assert_eq!(out_lines, exp_lines);

    // test shortlog command with -s -e options
    let args = ShortlogArgs::try_parse_from(["libra", "-s", "-e"]).unwrap();
    // shortlog::execute_to(args, &mut std::io::stdout()).await;
    let mut buf = Vec::new();
    shortlog::execute_to(args, &mut buf).await;
    let output = String::from_utf8(buf).unwrap();

    let expected = r#"   1  GUXUE <guxue@oa.org>
   5  LEAVE <leave@oa.org>
   1  LENGSA <lengsa@oa.org>
   1  MMONK <mmonk@oa.org>
   2  SHY <shy@oa.org>
   2  SunZo <sunzo@oa.org>
"#;

    let out_lines: Vec<_> = output.lines().collect();
    let exp_lines: Vec<_> = expected.lines().collect();
    assert_eq!(out_lines, exp_lines);

    // test shortlog command with --since option
    let args = ShortlogArgs::try_parse_from(["libra", "--since", "2026-01-10"]).unwrap();
    // shortlog::execute_to(args, &mut std::io::stdout()).await;
    let mut buf = Vec::new();
    shortlog::execute_to(args, &mut buf).await;
    let output = String::from_utf8(buf).unwrap();

    let expected = r#"   2  LEAVE
      Commit_12
      Commit_11
   1  MMONK
      Commit_10
   1  SHY
      Commit_13
   1  SunZo
      Commit_14
"#;
    let out_lines: Vec<_> = output.lines().collect();
    let exp_lines: Vec<_> = expected.lines().collect();
    assert_eq!(out_lines, exp_lines);

    // test shortlog command with --until option
    let args = ShortlogArgs::try_parse_from(["libra", "--until", "2026-01-13"]).unwrap();
    // shortlog::execute_to(args, &mut std::io::stdout()).await;
    let mut buf = Vec::new();
    shortlog::execute_to(args, &mut buf).await;
    let output = String::from_utf8(buf).unwrap();

    let expected = r#"   1  GUXUE
      Commit_7
   5  LEAVE
      Commit_12
      Commit_11
      Commit_4
      Commit_2
      Commit_1
   1  LENGSA
      Commit_8
   1  MMONK
      Commit_10
   2  SHY
      Commit_13
      Commit_5
   1  SunZo
      Commit_9
"#;

    let out_lines: Vec<_> = output.lines().collect();
    let exp_lines: Vec<_> = expected.lines().collect();
    assert_eq!(out_lines, exp_lines);

    // test shortlog command with comprehensive options
    let args = ShortlogArgs::try_parse_from([
        "libra",
        "-n",
        "-e",
        "--since",
        "2026-01-02",
        "--until",
        "2026-01-13",
    ])
    .unwrap();
    // shortlog::execute_to(args, &mut std::io::stdout()).await;
    let mut buf = Vec::new();
    shortlog::execute_to(args, &mut buf).await;
    let output = String::from_utf8(buf).unwrap();

    let expected = r#"   4  LEAVE <leave@oa.org>
      Commit_12
      Commit_11
      Commit_4
      Commit_2
   2  SHY <shy@oa.org>
      Commit_13
      Commit_5
   1  GUXUE <guxue@oa.org>
      Commit_7
   1  LENGSA <lengsa@oa.org>
      Commit_8
   1  MMONK <mmonk@oa.org>
      Commit_10
   1  SunZo <sunzo@oa.org>
      Commit_9
"#;

    let out_lines: Vec<_> = output.lines().collect();
    let exp_lines: Vec<_> = expected.lines().collect();
    assert_eq!(out_lines, exp_lines);
}
