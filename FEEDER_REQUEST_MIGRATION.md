# Feeder Request Migration Guide

## Overview

The new `feeder_request.rs` module unifies the functionality of `request_only_data.rs` and `request_data.rs` into a single, cleaner implementation.

## Key Differences

### Old Modules
- **`request_only_data.rs`**: Returns `ReturnData<T>` WITHOUT `jumlah` field
- **`request_data.rs`**: Returns `ReturnData<T>` WITH `jumlah: i32` field
- **`request_data_pagination.rs`**: Same as `request_data.rs` with `jumlah` field
- **`request_data_akumulasi.rs`**: Special case returning `StringOrInt` instead of `Vec<T>`

### New Unified Module
- **`feeder_request.rs`**: Returns `ReturnData<T>` with **optional** `jumlah: Option<i32>` field
- Handles both cases (with and without jumlah) seamlessly
- Uses `#[serde(skip_serializing_if = "Option::is_none")]` to handle optional fields cleanly

## Migration Steps

### 1. Update Imports

**Before:**
```rust
use crate::tasks::feeder_dikti::downstream::request_only_data::RequestData;
use crate::tasks::feeder_dikti::downstream::request_only_data::InputRequestData;
```

**After:**
```rust
use crate::tasks::feeder_dikti::downstream::feeder_request::{RequestData, InputRequestData};
```

### 2. Update API Calls

**Before (request_only_data.rs):**
```rust
let response = RequestData::get::<MyData>(
    ctx,
    InputRequestData {
        act: "GetSomeData".to_string(),
        filter: None,
        order: None,
        limit: None,
        offset: None,
    },
).await?;

// Access data
if let Some(data) = response.data {
    // No jumlah field available
}
```

**Before (request_data.rs):**
```rust
let response = RequestData::get::<MyData>(
    ctx,
    InputRequestData {
        act: "GetSomeData".to_string(),
        filter: None,
        order: None,
        limit: None,
        offset: None,
    },
).await?;

// Access data with jumlah
let total = response.jumlah; // i32
```

**After (feeder_request.rs):**
```rust
let response = RequestData::get::<MyData>(
    ctx,
    InputRequestData {
        act: "GetSomeData".to_string(),
        filter: None,
        order: None,
        limit: None,
        offset: None,
    },
).await?;

// Access data - jumlah is now optional
if let Some(total) = response.jumlah {
    println!("Total records: {}", total);
}

if let Some(data) = response.data {
    // Process data
}
```

## Benefits

### 1. **Single Source of Truth**
- One module to maintain instead of multiple similar ones
- Consistent API across all Feeder Dikti requests

### 2. **Flexible Response Handling**
- Automatically handles APIs that return `jumlah` or not
- No need for separate modules for different response formats

### 3. **Cleaner Code**
- Uses Serde's `skip_serializing_if` for optional fields
- Better documentation and examples
- Includes unit tests

### 4. **Nullable Parameters**
- All filter, order, limit, and offset parameters are properly optional
- Fields are skipped in JSON serialization when `None`

## Examples

### Example 1: Simple Request Without Pagination

```rust
use crate::tasks::feeder_dikti::downstream::feeder_request::{RequestData, InputRequestData};

async fn get_reference_data(ctx: &AppContext) -> Result<(), Error> {
    let response = RequestData::get::<ReferenceData>(
        ctx,
        InputRequestData {
            act: "GetAgama".to_string(),
            filter: None,
            order: None,
            limit: None,
            offset: None,
        },
    ).await?;

    if let Some(data) = response.data {
        for item in data {
            println!("Item: {:?}", item);
        }
    }

    Ok(())
}
```

### Example 2: Paginated Request

```rust
use crate::tasks::feeder_dikti::downstream::feeder_request::{RequestData, InputRequestData};

async fn get_paginated_data(ctx: &AppContext, offset: i32) -> Result<(), Error> {
    let response = RequestData::get::<StudentData>(
        ctx,
        InputRequestData {
            act: "GetListMahasiswa".to_string(),
            filter: Some("id_periode='20231'".to_string()),
            order: Some("nim ASC".to_string()),
            limit: Some(1000),
            offset: Some(offset),
        },
    ).await?;

    // Check if API has jumlah field
    if let Some(total) = response.jumlah {
        println!("Total records available: {}", total);
    }

    // Process data
    if let Some(data) = response.data {
        println!("Received {} records", data.len());
        for student in data {
            // Process each student
        }
    }

    Ok(())
}
```

### Example 3: Filtered Request

```rust
use crate::tasks::feeder_dikti::downstream::feeder_request::{RequestData, InputRequestData};

async fn get_filtered_data(ctx: &AppContext) -> Result<(), Error> {
    let response = RequestData::get::<KelasKuliah>(
        ctx,
        InputRequestData {
            act: "GetListKelasKuliah".to_string(),
            filter: Some("id_semester='20231' AND id_prodi='12345'".to_string()),
            order: Some("nama_kelas_kuliah ASC".to_string()),
            limit: Some(500),
            offset: Some(0),
        },
    ).await?;

    if response.error_code != 0 {
        if let Some(error_msg) = response.error_desc {
            eprintln!("API Error: {}", error_msg);
            return Err(Error::Message(error_msg));
        }
    }

    if let Some(data) = response.data {
        println!("Found {} classes", data.len());
    }

    Ok(())
}
```

## ReturnData Structure

```rust
pub struct ReturnData<T> {
    pub error_code: i32,              // 0 = success, non-zero = error
    pub error_desc: Option<String>,   // Error message if error_code != 0
    pub jumlah: Option<i32>,          // Total records (if provided by API)
    pub data: Option<Vec<T>>,         // Actual data array
}
```

## Error Handling

```rust
match RequestData::get::<MyData>(ctx, input).await {
    Ok(response) => {
        // Check API-level errors
        if response.error_code != 0 {
            if let Some(error_msg) = response.error_desc {
                eprintln!("API Error (code {}): {}", response.error_code, error_msg);
            }
        }
        
        // Process data
        if let Some(data) = response.data {
            // Success path
        } else {
            println!("No data returned (empty result)");
        }
    }
    Err(e) => {
        // HTTP or parsing errors
        eprintln!("Request failed: {}", e);
    }
}
```

## Checklist for Migration

- [ ] Replace imports from old modules to `feeder_request`
- [ ] Update `ReturnData` access patterns (check for `jumlah`)
- [ ] Ensure all `InputRequestData` instances use proper struct initialization
- [ ] Test that pagination still works correctly
- [ ] Verify error handling works as expected
- [ ] Remove old module imports once migration is complete

## Notes

- **`request_data_akumulasi.rs`** is NOT replaced by this module as it has a different response structure (`StringOrInt` instead of `Vec<T>`)
- The new module uses proper Serde attributes for cleaner JSON serialization
- All parameters (filter, order, limit, offset) are properly nullable
- The module includes unit tests for basic validation

## When to Use What

| Use Case | Module |
|----------|--------|
| Standard list/pagination requests | `feeder_request.rs` |
| Count/accumulation requests returning single value | `request_data_akumulasi.rs` |
| All other data fetching | `feeder_request.rs` |

## Deprecation Timeline

**Phase 1 (Current)**: 
- New `feeder_request.rs` available
- Old modules still functional

**Phase 2 (Recommended)**: 
- Gradually migrate existing code to use `feeder_request.rs`
- Update all new code to use the unified module

**Phase 3 (Future)**:
- Mark `request_only_data.rs` and `request_data.rs` as deprecated
- Remove old modules after full migration