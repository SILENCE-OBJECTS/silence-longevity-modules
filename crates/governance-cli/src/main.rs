use clap::{Parser, Subcommand};
use governance_effectlog::verify_chain;
use governance_validator::{
    filename_id, parse_record, publication_check, scan_s11, validate_record, GateError,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "governance-cli",
    about = "SILENCE open-core publication funnel"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Validate every markdown record under a tree.
    Validate { root: PathBuf },
    /// Intake gate for 00_inbox / records/inbox.
    IntakeCheck { root: PathBuf },
    /// Publication qualification (evidence, class, license, OWNER).
    PublicationCheck { root: PathBuf },
    /// Verify append-only EffectLog hash chain.
    VerifyChain { root: PathBuf },
}

fn collect_md(root: &Path) -> Result<Vec<PathBuf>, GateError> {
    let mut out = Vec::new();
    walk(root, &mut out)?;
    out.sort();
    Ok(out)
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), GateError> {
    if !dir.exists() {
        return Ok(());
    }
    let entries = fs::read_dir(dir).map_err(|e| GateError::Parse(e.to_string()))?;
    for e in entries {
        let e = e.map_err(|e| GateError::Parse(e.to_string()))?;
        let p = e.path();
        if p.is_dir() {
            walk(&p, out)?;
        } else if p.extension().and_then(|s| s.to_str()) == Some("md") {
            let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if name.ends_with("_TEMPLATE.md")
                || name == "README.md"
                || name == "INDEX.md"
                || name == "PUBLICATION_REGISTER.md"
            {
                continue;
            }
            out.push(p);
        }
    }
    Ok(())
}

fn load_records(
    root: &Path,
) -> Result<Vec<(PathBuf, governance_contracts::GovernanceRecord)>, GateError> {
    let mut recs = Vec::new();
    for p in collect_md(root)? {
        let text = fs::read_to_string(&p).map_err(|e| GateError::Parse(e.to_string()))?;
        if !text.starts_with("---") && !text.starts_with("\u{feff}---") {
            continue;
        }
        let rec = parse_record(&text)?;
        recs.push((p, rec));
    }
    Ok(recs)
}

fn cmd_validate(root: &Path) -> Result<(), GateError> {
    let recs = load_records(root)?;
    if recs.is_empty() {
        println!("validate: 0 records (ok)");
        return Ok(());
    }
    for (p, rec) in &recs {
        filename_id(p)?;
        validate_record(rec, Some(p))?;
        println!("OK  {}  {}", rec.status.as_str(), rec.id);
    }
    println!("validate: {} records", recs.len());
    Ok(())
}

fn cmd_intake(root: &Path) -> Result<(), GateError> {
    let recs = load_records(root)?;
    for (p, rec) in &recs {
        validate_record(rec, Some(p))?;
        scan_s11(&rec.body)?;
        if rec.kind != governance_contracts::RecordKind::Inbox
            && rec.kind != governance_contracts::RecordKind::Submission
        {
            return Err(GateError::Publication(format!(
                "intake expects IN/WN, got {}",
                rec.id
            )));
        }
        println!("INTAKE OK  {}", rec.id);
    }
    println!("intake-check: {} records", recs.len());
    Ok(())
}

fn cmd_pub(root: &Path) -> Result<(), GateError> {
    let recs = load_records(root)?;
    if recs.is_empty() {
        println!("publication-check: 0 records (nothing to qualify)");
        return Ok(());
    }
    for (_p, rec) in &recs {
        publication_check(rec, &rec.body)?;
        println!("PUB OK  {}", rec.id);
    }
    Ok(())
}

fn cmd_chain(root: &Path) -> Result<(), GateError> {
    let mut recs: Vec<_> = load_records(root)?.into_iter().map(|(_, r)| r).collect();
    recs.sort_by(|a, b| a.created.cmp(&b.created).then(a.id.cmp(&b.id)));
    let chain = verify_chain(&recs).map_err(GateError::Parse)?;
    for e in chain {
        println!("{}  {}", e.id, e.chain_hash);
    }
    Ok(())
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let res = match cli.cmd {
        Cmd::Validate { root } => cmd_validate(&root),
        Cmd::IntakeCheck { root } => cmd_intake(&root),
        Cmd::PublicationCheck { root } => cmd_pub(&root),
        Cmd::VerifyChain { root } => cmd_chain(&root),
    };
    match res {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("GATE FAIL: {e}");
            ExitCode::from(2)
        }
    }
}
