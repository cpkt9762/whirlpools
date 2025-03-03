//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshSerialize;
use borsh::BorshDeserialize;

/// Accounts.
#[derive(Debug)]
pub struct InitializeRewardV2 {
      
              
          pub reward_authority: solana_program::pubkey::Pubkey,
          
              
          pub funder: solana_program::pubkey::Pubkey,
          
              
          pub whirlpool: solana_program::pubkey::Pubkey,
          
              
          pub reward_mint: solana_program::pubkey::Pubkey,
          
              
          pub reward_token_badge: solana_program::pubkey::Pubkey,
          
              
          pub reward_vault: solana_program::pubkey::Pubkey,
          
              
          pub reward_token_program: solana_program::pubkey::Pubkey,
          
              
          pub system_program: solana_program::pubkey::Pubkey,
          
              
          pub rent: solana_program::pubkey::Pubkey,
      }

impl InitializeRewardV2 {
  pub fn instruction(&self, args: InitializeRewardV2InstructionArgs) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(args, &[])
  }
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, args: InitializeRewardV2InstructionArgs, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(9+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.reward_authority,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.funder,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.whirlpool,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.reward_mint,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.reward_token_badge,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.reward_vault,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.reward_token_program,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.rent,
            false
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let mut data = borsh::to_vec(&InitializeRewardV2InstructionData::new()).unwrap();
          let mut args = borsh::to_vec(&args).unwrap();
      data.append(&mut args);
    
    solana_program::instruction::Instruction {
      program_id: crate::WHIRLPOOL_ID,
      accounts,
      data,
    }
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
 pub struct InitializeRewardV2InstructionData {
            discriminator: [u8; 8],
            }

impl InitializeRewardV2InstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: [91, 1, 77, 50, 235, 229, 133, 49],
                                }
  }
}

impl Default for InitializeRewardV2InstructionData {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
 pub struct InitializeRewardV2InstructionArgs {
                  pub reward_index: u8,
      }


/// Instruction builder for `InitializeRewardV2`.
///
/// ### Accounts:
///
                ///   0. `[signer]` reward_authority
                      ///   1. `[writable, signer]` funder
                ///   2. `[writable]` whirlpool
          ///   3. `[]` reward_mint
          ///   4. `[]` reward_token_badge
                      ///   5. `[writable, signer]` reward_vault
          ///   6. `[]` reward_token_program
                ///   7. `[optional]` system_program (default to `11111111111111111111111111111111`)
                ///   8. `[optional]` rent (default to `SysvarRent111111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct InitializeRewardV2Builder {
            reward_authority: Option<solana_program::pubkey::Pubkey>,
                funder: Option<solana_program::pubkey::Pubkey>,
                whirlpool: Option<solana_program::pubkey::Pubkey>,
                reward_mint: Option<solana_program::pubkey::Pubkey>,
                reward_token_badge: Option<solana_program::pubkey::Pubkey>,
                reward_vault: Option<solana_program::pubkey::Pubkey>,
                reward_token_program: Option<solana_program::pubkey::Pubkey>,
                system_program: Option<solana_program::pubkey::Pubkey>,
                rent: Option<solana_program::pubkey::Pubkey>,
                        reward_index: Option<u8>,
        __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitializeRewardV2Builder {
  pub fn new() -> Self {
    Self::default()
  }
            #[inline(always)]
    pub fn reward_authority(&mut self, reward_authority: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.reward_authority = Some(reward_authority);
                    self
    }
            #[inline(always)]
    pub fn funder(&mut self, funder: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.funder = Some(funder);
                    self
    }
            #[inline(always)]
    pub fn whirlpool(&mut self, whirlpool: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.whirlpool = Some(whirlpool);
                    self
    }
            #[inline(always)]
    pub fn reward_mint(&mut self, reward_mint: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.reward_mint = Some(reward_mint);
                    self
    }
            #[inline(always)]
    pub fn reward_token_badge(&mut self, reward_token_badge: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.reward_token_badge = Some(reward_token_badge);
                    self
    }
            #[inline(always)]
    pub fn reward_vault(&mut self, reward_vault: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.reward_vault = Some(reward_vault);
                    self
    }
            #[inline(always)]
    pub fn reward_token_program(&mut self, reward_token_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.reward_token_program = Some(reward_token_program);
                    self
    }
            /// `[optional account, default to '11111111111111111111111111111111']`
#[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.system_program = Some(system_program);
                    self
    }
            /// `[optional account, default to 'SysvarRent111111111111111111111111111111111']`
#[inline(always)]
    pub fn rent(&mut self, rent: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.rent = Some(rent);
                    self
    }
                    #[inline(always)]
      pub fn reward_index(&mut self, reward_index: u8) -> &mut Self {
        self.reward_index = Some(reward_index);
        self
      }
        /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: solana_program::instruction::AccountMeta) -> &mut Self {
    self.__remaining_accounts.push(account);
    self
  }
  /// Add additional accounts to the instruction.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[solana_program::instruction::AccountMeta]) -> &mut Self {
    self.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[allow(clippy::clone_on_copy)]
  pub fn instruction(&self) -> solana_program::instruction::Instruction {
    let accounts = InitializeRewardV2 {
                              reward_authority: self.reward_authority.expect("reward_authority is not set"),
                                        funder: self.funder.expect("funder is not set"),
                                        whirlpool: self.whirlpool.expect("whirlpool is not set"),
                                        reward_mint: self.reward_mint.expect("reward_mint is not set"),
                                        reward_token_badge: self.reward_token_badge.expect("reward_token_badge is not set"),
                                        reward_vault: self.reward_vault.expect("reward_vault is not set"),
                                        reward_token_program: self.reward_token_program.expect("reward_token_program is not set"),
                                        system_program: self.system_program.unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
                                        rent: self.rent.unwrap_or(solana_program::pubkey!("SysvarRent111111111111111111111111111111111")),
                      };
          let args = InitializeRewardV2InstructionArgs {
                                                              reward_index: self.reward_index.clone().expect("reward_index is not set"),
                                    };
    
    accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
  }
}

  /// `initialize_reward_v2` CPI accounts.
  pub struct InitializeRewardV2CpiAccounts<'a, 'b> {
          
                    
              pub reward_authority: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub funder: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub whirlpool: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub reward_mint: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub reward_token_badge: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub reward_vault: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub reward_token_program: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub rent: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `initialize_reward_v2` CPI instruction.
pub struct InitializeRewardV2Cpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
      
              
          pub reward_authority: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub funder: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub whirlpool: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub reward_mint: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub reward_token_badge: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub reward_vault: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub reward_token_program: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub rent: &'b solana_program::account_info::AccountInfo<'a>,
            /// The arguments for the instruction.
    pub __args: InitializeRewardV2InstructionArgs,
  }

impl<'a, 'b> InitializeRewardV2Cpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: InitializeRewardV2CpiAccounts<'a, 'b>,
              args: InitializeRewardV2InstructionArgs,
      ) -> Self {
    Self {
      __program: program,
              reward_authority: accounts.reward_authority,
              funder: accounts.funder,
              whirlpool: accounts.whirlpool,
              reward_mint: accounts.reward_mint,
              reward_token_badge: accounts.reward_token_badge,
              reward_vault: accounts.reward_vault,
              reward_token_program: accounts.reward_token_program,
              system_program: accounts.system_program,
              rent: accounts.rent,
                    __args: args,
          }
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], &[])
  }
  #[inline(always)]
  pub fn invoke_with_remaining_accounts(&self, remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
  }
  #[inline(always)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed_with_remaining_accounts(
    &self,
    signers_seeds: &[&[&[u8]]],
    remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]
  ) -> solana_program::entrypoint::ProgramResult {
    let mut accounts = Vec::with_capacity(9+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.reward_authority.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.funder.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.whirlpool.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.reward_mint.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.reward_token_badge.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.reward_vault.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.reward_token_program.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.rent.key,
            false
          ));
                      remaining_accounts.iter().for_each(|remaining_account| {
      accounts.push(solana_program::instruction::AccountMeta {
          pubkey: *remaining_account.0.key,
          is_signer: remaining_account.1,
          is_writable: remaining_account.2,
      })
    });
    let mut data = borsh::to_vec(&InitializeRewardV2InstructionData::new()).unwrap();
          let mut args = borsh::to_vec(&self.__args).unwrap();
      data.append(&mut args);
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::WHIRLPOOL_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(10 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.reward_authority.clone());
                        account_infos.push(self.funder.clone());
                        account_infos.push(self.whirlpool.clone());
                        account_infos.push(self.reward_mint.clone());
                        account_infos.push(self.reward_token_badge.clone());
                        account_infos.push(self.reward_vault.clone());
                        account_infos.push(self.reward_token_program.clone());
                        account_infos.push(self.system_program.clone());
                        account_infos.push(self.rent.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `InitializeRewardV2` via CPI.
///
/// ### Accounts:
///
                ///   0. `[signer]` reward_authority
                      ///   1. `[writable, signer]` funder
                ///   2. `[writable]` whirlpool
          ///   3. `[]` reward_mint
          ///   4. `[]` reward_token_badge
                      ///   5. `[writable, signer]` reward_vault
          ///   6. `[]` reward_token_program
          ///   7. `[]` system_program
          ///   8. `[]` rent
#[derive(Clone, Debug)]
pub struct InitializeRewardV2CpiBuilder<'a, 'b> {
  instruction: Box<InitializeRewardV2CpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitializeRewardV2CpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(InitializeRewardV2CpiBuilderInstruction {
      __program: program,
              reward_authority: None,
              funder: None,
              whirlpool: None,
              reward_mint: None,
              reward_token_badge: None,
              reward_vault: None,
              reward_token_program: None,
              system_program: None,
              rent: None,
                                            reward_index: None,
                    __remaining_accounts: Vec::new(),
    });
    Self { instruction }
  }
      #[inline(always)]
    pub fn reward_authority(&mut self, reward_authority: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.reward_authority = Some(reward_authority);
                    self
    }
      #[inline(always)]
    pub fn funder(&mut self, funder: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.funder = Some(funder);
                    self
    }
      #[inline(always)]
    pub fn whirlpool(&mut self, whirlpool: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.whirlpool = Some(whirlpool);
                    self
    }
      #[inline(always)]
    pub fn reward_mint(&mut self, reward_mint: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.reward_mint = Some(reward_mint);
                    self
    }
      #[inline(always)]
    pub fn reward_token_badge(&mut self, reward_token_badge: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.reward_token_badge = Some(reward_token_badge);
                    self
    }
      #[inline(always)]
    pub fn reward_vault(&mut self, reward_vault: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.reward_vault = Some(reward_vault);
                    self
    }
      #[inline(always)]
    pub fn reward_token_program(&mut self, reward_token_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.reward_token_program = Some(reward_token_program);
                    self
    }
      #[inline(always)]
    pub fn system_program(&mut self, system_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.system_program = Some(system_program);
                    self
    }
      #[inline(always)]
    pub fn rent(&mut self, rent: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.rent = Some(rent);
                    self
    }
                    #[inline(always)]
      pub fn reward_index(&mut self, reward_index: u8) -> &mut Self {
        self.instruction.reward_index = Some(reward_index);
        self
      }
        /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: &'b solana_program::account_info::AccountInfo<'a>, is_writable: bool, is_signer: bool) -> &mut Self {
    self.instruction.__remaining_accounts.push((account, is_writable, is_signer));
    self
  }
  /// Add additional accounts to the instruction.
  ///
  /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
  /// and a `bool` indicating whether the account is a signer or not.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> &mut Self {
    self.instruction.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed(&[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
          let args = InitializeRewardV2InstructionArgs {
                                                              reward_index: self.instruction.reward_index.clone().expect("reward_index is not set"),
                                    };
        let instruction = InitializeRewardV2Cpi {
        __program: self.instruction.__program,
                  
          reward_authority: self.instruction.reward_authority.expect("reward_authority is not set"),
                  
          funder: self.instruction.funder.expect("funder is not set"),
                  
          whirlpool: self.instruction.whirlpool.expect("whirlpool is not set"),
                  
          reward_mint: self.instruction.reward_mint.expect("reward_mint is not set"),
                  
          reward_token_badge: self.instruction.reward_token_badge.expect("reward_token_badge is not set"),
                  
          reward_vault: self.instruction.reward_vault.expect("reward_vault is not set"),
                  
          reward_token_program: self.instruction.reward_token_program.expect("reward_token_program is not set"),
                  
          system_program: self.instruction.system_program.expect("system_program is not set"),
                  
          rent: self.instruction.rent.expect("rent is not set"),
                          __args: args,
            };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct InitializeRewardV2CpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            reward_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                funder: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                whirlpool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                reward_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                reward_token_badge: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                reward_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                reward_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                        reward_index: Option<u8>,
        /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}

