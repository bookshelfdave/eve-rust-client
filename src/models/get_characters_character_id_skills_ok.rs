/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdSkillsOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdSkillsOk {
  /// skills array
  #[serde(rename = "skills")]
  skills: Vec<::models::GetCharactersCharacterIdSkillsSkill>,
  /// total_sp integer
  #[serde(rename = "total_sp")]
  total_sp: i64,
  /// Skill points available to be assigned
  #[serde(rename = "unallocated_sp")]
  unallocated_sp: Option<i32>
}

impl GetCharactersCharacterIdSkillsOk {
  /// 200 ok object
  pub fn new(skills: Vec<::models::GetCharactersCharacterIdSkillsSkill>, total_sp: i64) -> GetCharactersCharacterIdSkillsOk {
    GetCharactersCharacterIdSkillsOk {
      skills: skills,
      total_sp: total_sp,
      unallocated_sp: None
    }
  }

  pub fn set_skills(&mut self, skills: Vec<::models::GetCharactersCharacterIdSkillsSkill>) {
    self.skills = skills;
  }

  pub fn with_skills(mut self, skills: Vec<::models::GetCharactersCharacterIdSkillsSkill>) -> GetCharactersCharacterIdSkillsOk {
    self.skills = skills;
    self
  }

  pub fn skills(&self) -> &Vec<::models::GetCharactersCharacterIdSkillsSkill> {
    &self.skills
  }


  pub fn set_total_sp(&mut self, total_sp: i64) {
    self.total_sp = total_sp;
  }

  pub fn with_total_sp(mut self, total_sp: i64) -> GetCharactersCharacterIdSkillsOk {
    self.total_sp = total_sp;
    self
  }

  pub fn total_sp(&self) -> &i64 {
    &self.total_sp
  }


  pub fn set_unallocated_sp(&mut self, unallocated_sp: i32) {
    self.unallocated_sp = Some(unallocated_sp);
  }

  pub fn with_unallocated_sp(mut self, unallocated_sp: i32) -> GetCharactersCharacterIdSkillsOk {
    self.unallocated_sp = Some(unallocated_sp);
    self
  }

  pub fn unallocated_sp(&self) -> Option<&i32> {
    self.unallocated_sp.as_ref()
  }

  pub fn reset_unallocated_sp(&mut self) {
    self.unallocated_sp = None;
  }

}



