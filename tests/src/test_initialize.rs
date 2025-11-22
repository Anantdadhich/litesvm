
    use {
        anchor_client::{anchor_lang::Key, solana_sdk::{msg, pubkey::Pubkey}}, anchor_lang::{
            AccountDeserialize, InstructionData, ToAccountMetas, prelude::msg, solana_program::program_pack::Pack
        }, anchor_spl::{
            associated_token::{
                self, 
                spl_associated_token_account
            }, 
            token::spl_token
        }, litesvm::LiteSVM, litesvm_token::{
            CreateAssociatedTokenAccount, CreateMint, MintTo, spl_token::ID as TOKEN_PROGRAM_ID
        }, solana_account::Account, solana_address::Address, solana_instruction::Instruction, solana_keypair::Keypair, solana_message::Message, solana_native_token::LAMPORTS_PER_SOL, solana_pubkey::Pubkey, solana_rpc_client::rpc_client::RpcClient, solana_sdk_ids::system_program::ID as SYSTEM_PROGRAM_ID, solana_signer::Signer, solana_transaction::Transaction, std::{
            path::PathBuf, 
            str::FromStr
        }
    };

    static PROGRAM_ID:Pubkey=crate::ID;

fn setup()->(Litesvm,keypair){
    let mut program=Litesvm::new();

    let payer=Keypair::new();
     //airdrop some sol
    program.airdrop(&payer.pubkey(),10 * LAMPORTS_PER_SOL).expects("Failed to airdrop sol to payer ");

      let so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/deploy/litesvm.so");
    
 
    let program_data=std::fs::read(so_path).expect("Failed to read program so file ");

    program.add_program(PROGRAM_ID,&program_data);

     let rpc_client = RpcClient::new("https://api.devnet.solana.com");
        let account_address = Address::from_str("E6zstiTDc2K1GN3yrUJET2xrhQDKSFMmT8EcuvuWdiWg").unwrap();
        let fetched_account = rpc_client
            .get_account(&account_address)
            .expect("Failed to fetch account from devnet");

    program.set_account(payer.pubkey(),Account{
        lamports:fetched_account.lamports,
        data:fetched_account.data,
        owner:Pubkey::from(fetched_account.owner.to_bytes()),
        executable:fetched_account.executable,
        rent_epoch:fetched_account.rent_epoch
    }).unwrap();


    msg!("Lamports of fetched account {}",fetched_account.lamports);

    (program,payer)
}

#[test]
fn test_make(){
     let (mut program,payer)=setup();
     let maker =payer.pubkey();

     let mint_a=CreateMint::new(&mut program, &payer).decimals(6).authority(&maker).send().unwrap();
     msg!("Mint A:{}",mint_a);

     let mint_b=CreateMint::new(&mut program, &payer).decimals(6).authority(&maker).send().unwrap();
     msg!("Mint B:{}",mint_b);

     let maker_ata_a=CreateAssociatedTokenAccount::new(&mut program, &payer, &mint_a);
     msg!("maker ata a :{}",maker_ata_a);

     let escrow=Pubkey::find_program_address(&[b"escrow", maker.key().as_ref(),&123u64.to_le_bytes()],&PROGRAM_ID).0;
     msg!("Escrow pda : {}",escrow);

     let vault=associated_token::get_associated_token_address(&escrow,&mint_a);
     msg!("Vault pda :{}",vault);

     let associated_token_program=spl_associated_token_account::ID;

     let token_program=TOKEN_PROGRAM_ID;
     let system_program=SYSTEM_PROGRAM_ID;


     MintTo::new(&mut program,&payer,&mint_a,&maker_ata_a,1000000000).send().unwrap();

     let make_ix=Instruction{
        program_id:PROGRAM_ID,
        accounts:crate::accounts::Make {
             maker:maker,
             mint_a:mint_a,
             mint_b:mint_b,
             maker_ata_a:maker_ata_a,
             escrow:escrow,
             vault:vault,
             associated_token_program:associated_token_program,
             token_program:token_program,
             system_program:system_program,
             
        }.to_account_metas(None),
        data:crate::instruction::Make{
            deposit:10,
            seed:123u64,
            recieve:10
        }.data(),
     };

     let message=Message::new(&[makeix], Some(&payer.pubkey));

     let receent_blockhash=program.latest_blockhash();

        let transaction = Transaction::new(&[&payer], message, recent_blockhash);

       
        let tx = program.send_transaction(transaction).unwrap();

       
        msg!("\n\nMake transaction sucessfull");
        msg!("CUs Consumed: {}", tx.compute_units_consumed);
        msg!("Tx Signature: {}", tx.signature);

        
        let vault_account = program.get_account(&vault).unwrap();
        let vault_data = spl_token::state::Account::unpack(&vault_account.data).unwrap();
        assert_eq!(vault_data.amount, 10);
        assert_eq!(vault_data.owner, escrow);
        assert_eq!(vault_data.mint, mint_a);

        let escrow_account = program.get_account(&escrow).unwrap();
        let escrow_data = crate::state::Escrow::try_deserialize(&mut escrow_account.data.as_ref()).unwrap();
        assert_eq!(escrow_data.seed, 123u64);
        assert_eq!(escrow_data.maker, maker);
        assert_eq!(escrow_data.mint_a, mint_a);
        assert_eq!(escrow_data.mint_b, mint_b);
        assert_eq!(escrow_data.receive, 10);



}