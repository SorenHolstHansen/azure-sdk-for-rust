// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.
// Code generated by Microsoft (R) Rust Code Generator. DO NOT EDIT.

use typespec_client_core::{create_enum, create_extensible_enum};

create_extensible_enum!(
    DeletionRecoveryLevel,
    (CustomizedRecoverable, "CustomizedRecoverable"),
    (
        CustomizedRecoverableProtectedSubscription,
        "CustomizedRecoverable+ProtectedSubscription"
    ),
    (
        CustomizedRecoverablePurgeable,
        "CustomizedRecoverable+Purgeable"
    ),
    (Purgeable, "Purgeable"),
    (Recoverable, "Recoverable"),
    (
        RecoverableProtectedSubscription,
        "Recoverable+ProtectedSubscription"
    ),
    (RecoverablePurgeable, "Recoverable+Purgeable")
);
