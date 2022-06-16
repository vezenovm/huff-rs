use std::path::PathBuf;

use huff_codegen::Codegen;
use huff_core::*;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[ignore]
#[test]
fn test_erc721_compile() {
    let file_sources: Vec<FileSource> = Compiler::fetch_sources(&vec![PathBuf::from(
        "../huff-examples/erc721/contracts/ERC721.huff".to_string(),
    )]);

    // Recurse file deps + generate flattened source
    let file_source = file_sources.get(0).unwrap();
    let recursed_file_source = Compiler::recurse_deps(file_source.clone()).unwrap();
    let flattened = recursed_file_source.fully_flatten();
    let full_source = FullFileSource {
        source: &flattened.0,
        file: Some(file_source.clone()),
        spans: flattened.1,
    };
    let lexer = Lexer::new(full_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("../huff-examples/erc20/contracts".to_string()));
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    // Create main and constructor bytecode
    let main_bytecode = Codegen::roll(Some(contract.clone())).unwrap();
    let constructor_bytecode = Codegen::construct(Some(contract.clone())).unwrap();

    // Churn
    let mut cg = Codegen::new();
    let artifact =
        cg.churn(file_source.clone(), vec![], &main_bytecode, &constructor_bytecode).unwrap();

    // Full expected bytecode output (generated from huffc)
    let expected_bytecode = "336000556103b1806100116000396000f360003560E01c8063a9059cbb146100a057806342842e0e146101a3578063b88d4fde146101a9578063095ea7b31461027b578063a22cb46514610310578063081812fc146102f357806340c10f19146101af57806370a082311461025e5780636352211e1461039457806306fdde031461035e57806395d89b4114610364578063c87b56dd1461036a57806301ffc9a714610370578063e985e9c514610376575b6044356024356004358083600160005260006020015260406000205491146100c75761019d565b8033146101005733816000526000602001526040600020546101005782600360005260006020015260406000205433146101005761019d565b6001816002600052600060200152604060002054038160026000526000602001526040600020558160026000526000602001526040600020546001018260026000526000602001526040600020558183600160005260006020015260406000205560008360036000526000602001526040600020557fDDF252AD1BE2C89B69C2B068FC378DAA952BA7F163C4A11628F55A4DF523B3EF60206000a4005b60006000fd5b60006000fd5b60006000fd5b60005433146101be5760006000fd5b6024356004356000826001600052600060200152604060002054156101e257610258565b8160026000526000602001526040600020546001018260026000526000602001526040600020558183600160005260006020015260406000205560008360036000526000602001526040600020557fDDF252AD1BE2C89B69C2B068FC378DAA952BA7F163C4A11628F55A4DF523B3EF60006000a4005b60006000fd5b600435600260005260006020015260406000205460005260206000f35b6024358060016000526000602001526040600020548033143382600052600060200152604060002054176102ae576102ed565b60043580836003600052600060200152604060002055907f8C5BE1E5EBEC7D5BD14F71427D1E84F3DD0314C0F7B2291E5B200AC8C7C3B92560006000a4005b60006000fd5b600435600360005260006020015260406000205460005260206000f35b60243560043533600052600060200152604060002055600435336024356000527f17307EAB39AB6107E8899845AD3D59BD9653F200F220920489CA2B5937696C3160006000a4005b60006000fd5b60006000fd5b60006000fd5b60006000fd5b60006000fd5b60243560043560005260006020015260406000205460005260206000f35b600435600160005260006020015260406000205460005260206000f3";
    let _current_bytecode = "";

    println!("Expected bytecode: {}", expected_bytecode.to_lowercase());
    println!("Current bytecode: {}", artifact.bytecode.to_lowercase());

    // TODO: Check the bytecode
    // assert_eq!(artifact.bytecode.to_lowercase(), expected_bytecode.to_lowercase());
}
