//! Display output as a valid 'unified' format patch.

use owo_colors::OwoColorize;

use crate::{
    context::all_matched_lines_filled,
    hunks::{matched_lines_for_hunk, Hunk},
    side_by_side::{lines_with_novel, split_on_newlines},
    style::{apply_colors, BackgroundColor},
    syntax::MatchedPos,
};

fn print_file_header(lhs_path: &str, rhs_path: &str, use_color: bool) {
    let mut lhs_path_pretty = format!("--- {}", lhs_path);
    if use_color {
        lhs_path_pretty = lhs_path_pretty.yellow().bold().to_string();
    }
    let mut rhs_path_pretty = format!("+++ {}", rhs_path);
    if use_color {
        rhs_path_pretty = rhs_path_pretty.yellow().bold().to_string();
    }
    println!("{}", lhs_path_pretty);
    println!("{}", rhs_path_pretty);
}

pub fn print(
    hunks: &[Hunk],
    lhs_path: &str,
    rhs_path: &str,
    use_color: bool,
    background: BackgroundColor,
    lhs_src: &str,
    rhs_src: &str,
    lhs_mps: &[MatchedPos],
    rhs_mps: &[MatchedPos],
) {
    print_file_header(lhs_path, rhs_path, use_color);

    let (lhs_colored_src, rhs_colored_src) = if use_color {
        (
            apply_colors(lhs_src, true, background, lhs_mps),
            apply_colors(rhs_src, false, background, rhs_mps),
        )
    } else {
        (lhs_src.to_string(), rhs_src.to_string())
    };

    let lhs_lines = split_on_newlines(lhs_src);
    let rhs_lines = split_on_newlines(rhs_src);
    let lhs_colored_lines = split_on_newlines(&lhs_colored_src);
    let rhs_colored_lines = split_on_newlines(&rhs_colored_src);

    let (lhs_lines_with_novel, rhs_lines_with_novel) = lines_with_novel(lhs_mps, rhs_mps);

    let matched_lines = all_matched_lines_filled(lhs_mps, rhs_mps, &lhs_lines, &rhs_lines);

    for hunk in hunks {
        println!("{}", "@@ -1,99, +2,100 @@".yellow().bold());

        let aligned_lines = matched_lines_for_hunk(&matched_lines, hunk);
        for (lhs_line_num, _) in &aligned_lines {
            if let Some(lhs_line_num) = &lhs_line_num {
                let lhs_line = &lhs_colored_lines[lhs_line_num.0];

                if lhs_lines_with_novel.contains(lhs_line_num) {
                    break;
                }

                println!(" {}", lhs_line);
            }
        }
        for (_, rhs_line_num) in &aligned_lines {
            if let Some(rhs_line_num) = rhs_line_num {
                if rhs_lines_with_novel.contains(rhs_line_num) {
                    let rhs_line = &rhs_colored_lines[rhs_line_num.0];
                    println!("+{}", rhs_line);
                }
            }
        }
        let mut seen_novel = false;
        for (lhs_line_num, _) in &aligned_lines {
            if let Some(lhs_line_num) = &lhs_line_num {
                let is_novel = lhs_lines_with_novel.contains(lhs_line_num);
                if !seen_novel {
                    if is_novel {
                        seen_novel = true;
                    } else {
                        continue;
                    }
                }

                let lhs_line = &lhs_colored_lines[lhs_line_num.0];
                println!("{}{}", if is_novel { "+" } else { " " }, lhs_line);
            }
        }
    }
}
