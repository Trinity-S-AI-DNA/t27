use std::{fs,process::Command};
pub fn run(cmd:&crate::GitAction)->anyhow::Result<i32>{
 let root=crate::find_trinity_root()?;
 match cmd {
  crate::GitAction::Commit{all,message,..}=>{
   let s=fs::read_to_string(root.join(".trinity/state/active-skill.json")).unwrap_or_default();
   let present=s.contains("\"status\"");
   let issue=s.lines().find(|l|l.contains("issue_id")).and_then(|l|l.split('"').nth(3)).unwrap_or("").to_string();
   let toxic=s.contains("TOXIC");
   let g=crate::gtw::commit_gate(present,!issue.is_empty(),toxic);
   if g!=crate::gtw::EXIT_OK{eprintln!("exit {g}");return Ok(g as i32);}
   if *all{let _=Command::new("git").args(["add","-A"]).status()?;}
   let mut m=message.clone().unwrap_or_default();
   let has=crate::gtw::message_references_issue(m.contains("Closes #"),m.contains("Fixes #"),m.contains("Resolves #"),m.contains("Reference #"));
   if !has&&!issue.is_empty(){m=format!("{} Closes #{}",m,issue);}
   let o=Command::new("git").args(["commit","-m",&m]).status()?;
   Ok(if o.success(){0}else{1})
  }
  crate::GitAction::Push{remote,branch,..}=>{
   let c=fs::read_to_string(root.join(".trinity/cells/registry.json")).unwrap_or_default();
   let active=c.matches("\"state\":").count() as u8 - c.matches("\"sealed\"").count() as u8;
   let g=crate::gtw::push_gate(active,active==0,false,crate::gtw::KIND_NORMAL,0,false,false);
   if g!=crate::gtw::EXIT_OK{eprintln!("exit {g}");return Ok(g as i32);}
   let r=remote.clone().unwrap_or_else(||"origin".into());
   let b=branch.clone().unwrap_or_else(||{let o=Command::new("git").args(["rev-parse","--abbrev-ref","HEAD"]).output().ok();String::from_utf8_lossy(&o.map(|x|x.stdout).unwrap_or_default()).trim().to_string()});
   let o=Command::new("git").args(["push",&r,&b]).status()?;
   Ok(if o.success(){0}else{2})
  }
  crate::GitAction::Status=>{
   let o=Command::new("git").args(["status","--porcelain"]).output()?;
   let t=String::from_utf8_lossy(&o.stdout);
   let (mut m,mut a,mut d,mut u)=(0u32,0,0,0);
   for l in t.lines(){match l.chars().next().unwrap_or(' '){'M'=>m+=1,'A'=>a+=1,'D'=>d+=1,'?'=>u+=1,_=>{}}}
   println!("modified={} added={} deleted={} untracked={} clean={}",m,a,d,u,crate::gtw::is_clean(m,a,d,u));
   Ok(0)
  }
 }
}