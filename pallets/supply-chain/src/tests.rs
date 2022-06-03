// Copyright 2021-2022 Green Aureus GmbH

// Permission is hereby granted, free of charge, to any person obtaining a copy 
// of this software and associated documentation files (the "Software"), to read 
// the Software only. Permission is hereby NOT GRANTED to use, copy, modify, 
// merge, publish, distribute, sublicense, and/or sell copies of the Software.

use crate::{mock::*, types::ComponentId, Components, Config, Error};
use frame_support::{assert_err, assert_ok};

fn setup_authorities(num: u8) -> Vec<u8> {
    let range: Vec<u8> = (0..num).collect();

    for auth_id in range.iter() {
        assert_ok!(SupplyChain::authorize(
            Origin::root(),
            u64::from(*auth_id),
            format!("AUTHORITY_{}", auth_id)
        ));
    }

    range
}

// Test: authorize()
#[test]
fn add_authority_adds_authority() {
    new_test_ext().execute_with(|| {
        let authority = 0;
        let org = "ExampleOrg".to_string();

        assert_ok!(SupplyChain::authorize(
            Origin::root(),
            authority,
            org.clone()
        ));
        assert_eq!(SupplyChain::authorities(authority), org);
    });
}

#[test]
fn add_authority_adds_authority_twice() {
    new_test_ext().execute_with(|| {
        assert_ok!(SupplyChain::authorize(
            Origin::root(),
            0,
            "ExampleOrg".to_string()
        ));
        assert_err!(
            SupplyChain::authorize(Origin::root(), 0, "ExampleOrg".to_string()),
            <Error<Test>>::AlreadyAuthorized
        );
    });
}

#[test]
fn add_authority_tries_to_add_empty_name() {
    new_test_ext().execute_with(|| {
        assert_err!(
            SupplyChain::authorize(Origin::root(), 0, "".to_string()),
            <Error<Test>>::EmptyDataProvided
        );
    });
}

#[test]
fn add_authority_tries_to_add_too_long_name() {
    new_test_ext().execute_with(|| {
        let long_name =
            "s".repeat((<Test as Config>::MaxAuditorNameLength::get() + 1).into());

        assert_err!(
            SupplyChain::authorize(Origin::root(), 0, long_name),
            <Error<Test>>::AuditorNameTooLong
        );
    });
}

// Test: deauthorize()
#[test]
fn remove_authority_removes_authority() {
    new_test_ext().execute_with(|| {
        let authority = 0;
        let org = "ExampleOrg".to_string();

        assert_ok!(SupplyChain::authorize(
            Origin::root(),
            authority,
            org.clone()
        ));
        assert_ok!(SupplyChain::deauthorize(Origin::root(), authority));
    });
}

#[test]
fn remove_authority_fails_removing_non_existing_authority() {
    new_test_ext().execute_with(|| {
        let authority = 0;

        assert_err!(
            SupplyChain::deauthorize(Origin::root(), authority),
            <Error<Test>>::AuthorityNotFound
        );
    });
}

// Test: checked_add_audit()
#[test]
fn add_audit_component_id_tool_long() {
    new_test_ext().execute_with(|| {
        let authority = setup_authorities(1).pop().unwrap().into();
        let audit_data = "1234".to_string();
        let components = None;
        let component_id: ComponentId =
            "0".repeat((<Test as Config>::MaxComponentIdLength::get() + 1).into());

        assert_err!(
            SupplyChain::checked_add_audit(authority, audit_data, components, component_id),
            <Error<Test>>::ComponentIdTooLong
        )
    });
}

#[test]
fn add_audit_no_data() {
    new_test_ext().execute_with(|| {
        let authority = setup_authorities(1).pop().unwrap().into();
        let audit_data = Default::default();
        let components = None;
        let component_id: ComponentId = "386a00b808e37a15".to_string();

        assert_err!(
            SupplyChain::checked_add_audit(authority, audit_data, components, component_id),
            <Error<Test>>::EmptyDataProvided
        )
    });
}

#[test]
fn add_audit_too_much_data() {
    new_test_ext().execute_with(|| {
        let authority = setup_authorities(1).pop().unwrap().into();
        let audit_data = "1".repeat((<Test as Config>::MaxAuditSize::get() + 1).into()).to_string();
        let components = None;
        let component_id: ComponentId = "386a00b808e37a15".to_string();

        assert_err!(
            SupplyChain::checked_add_audit(authority, audit_data, components, component_id),
            <Error<Test>>::AuditTooBig
        )
    });
}

#[test]
fn add_audit_auditor_not_authorized() {
    new_test_ext().execute_with(|| {
        let authority = 1;
        let audit_data = "1".to_string();
        let components = None;
        let component_id: ComponentId = "386a00b808e37a15".to_string();

        assert_err!(
            SupplyChain::checked_add_audit(authority, audit_data, components, component_id),
            <Error<Test>>::Unauthorized
        )
    });
}

#[test]
fn add_audit_too_many_audits() {
    new_test_ext().execute_with(|| {
        let authority = setup_authorities(1).pop().unwrap().into();
        let audit_data = "1".to_string();
        let components = None;
        let component_id: ComponentId = "386a00b808e37a15".to_string();

        for _ in 0..<Test as Config>::MaxAudits::get() {
            assert_ok!(SupplyChain::checked_add_audit(
                authority,
                audit_data.clone(),
                components.clone(),
                component_id.clone()
            ));
        }

        assert_err!(
            SupplyChain::checked_add_audit(authority, audit_data, components, component_id),
            <Error<Test>>::MaxAuditsReached
        )
    });
}

#[test]
fn add_assembly_audit_component_exists() {
    new_test_ext().execute_with(|| {
        let authority = setup_authorities(1).pop().unwrap().into();
        let audit_data = "1".to_string();
        let mut components = None;
        let component_id: ComponentId = "386a00b808e37a15".to_string();

        assert_ok!(SupplyChain::checked_add_audit(
            authority,
            audit_data.clone(),
            components.clone(),
            component_id.clone()
        ));

        components = Some(vec!["8f8915a158f4e5d7".to_string()]);
        assert_err!(
            SupplyChain::checked_add_audit(authority, audit_data, components, component_id),
            <Error<Test>>::ComponentAlreadyExists
        )
    });
}

#[test]
fn add_assembly_audit_too_many_components() {
    new_test_ext().execute_with(|| {
        let authority = setup_authorities(1).pop().unwrap().into();
        let audit_data = "1".to_string();
        let components = Some(vec![
            "1".to_string();
            (<Test as Config>::MaxComponents::get() as u32 + 1)
                as usize
        ]);
        let component_id: ComponentId = "386a00b808e37a15".to_string();

        assert_err!(
            SupplyChain::checked_add_audit(authority, audit_data, components, component_id),
            <Error<Test>>::MaxComponentsReached
        )
    });
}

#[test]
fn add_assembly_audit_too_many_component_of() {
    new_test_ext().execute_with(|| {
        let components = (0..<Test as Config>::MaxComponents::get()).into_iter().map(|v| v.to_string()).collect();

        <Components<Test>>::mutate("0", |comp| {
            comp.component_of = components;
        });

        let authority = setup_authorities(1).pop().unwrap().into();
        let audit_data = "1".to_string();
        let components = Some(vec!["0".to_string()]);
        let component_id: ComponentId = "386a00b808e37a15".to_string();

        assert_err!(
            SupplyChain::checked_add_audit(authority, audit_data, components, component_id),
            <Error<Test>>::MaxComponentOfReached
        )
    });
}

#[test]
fn add_audit_adds_audits() {
    new_test_ext().execute_with(|| {
        let authority = setup_authorities(1).pop().unwrap().into();
        let audit_data = "1".to_string();
        let components = None;
        let component_id: ComponentId = "386a00b808e37a15".to_string();

        assert_ok!(SupplyChain::checked_add_audit(
            authority,
            audit_data.clone(),
            components,
            component_id.clone()
        ));

        let now = <Test as Config>::Timestamp::now();
        let authority_name = SupplyChain::authorities(authority);
        let audit = &SupplyChain::components(component_id).audits[0];
        assert_eq!(audit.timestamp, now);
        assert_eq!(audit.auditor, authority_name);
        assert_eq!(audit.audit_data, audit_data);
    });
}

#[test]
fn add_assembly_audit_adds_components_and_audit() {
    new_test_ext().execute_with(|| {
        let authority = setup_authorities(1).pop().unwrap().into();
        let audit_data = "1".to_string();
        let comp_one = "0".to_string();
        let comp_two = "1".to_string();
        let components = Some(vec![comp_one.clone(), comp_two.clone()]);
        let component_id: ComponentId = "386a00b808e37a15".to_string();

        assert_ok!(SupplyChain::checked_add_audit(
            authority,
            audit_data.clone(),
            components.clone(),
            component_id.clone()
        ));

        let now = <Test as Config>::Timestamp::now();
        let authority_name = SupplyChain::authorities(authority);
        let created_component = &SupplyChain::components(component_id.clone());
        let audit = &created_component.audits[0];
        assert_eq!(audit.timestamp, now);
        assert_eq!(audit.auditor, authority_name);
        assert_eq!(audit.audit_data, audit_data);
        assert_eq!(created_component.components, components.unwrap().into_iter().collect());
        assert!(SupplyChain::components(comp_one).component_of.contains(&component_id));
        assert!(SupplyChain::components(comp_two).component_of.contains(&component_id));
    });
}