use crate::context::Context;
use crate::util::to_rust_str;
use crate::Result;
use odpi_sys::*;
use std::convert::TryInto;

mod object_attr;
mod object_type;

pub use object_attr::ObjectAttr;
pub use object_type::ObjectType;

dpi_enum! {
    /// native C types corresponding to [ODPI-C dpiNativeTypeNum][]
    ///
    /// [ODPI-C dpiNativeTypeNum]: https://oracle.github.io/odpi/doc/enums/dpiNativeTypeNum.html
    #[derive(Debug, Clone, PartialEq)]
    #[repr(u32)]
    pub enum NativeType : dpiNativeTypeNum {
        Int64 = DPI_NATIVE_TYPE_INT64,
        UInt64 = DPI_NATIVE_TYPE_UINT64,
        Float = DPI_NATIVE_TYPE_FLOAT,
        Double = DPI_NATIVE_TYPE_DOUBLE,
        Bytes = DPI_NATIVE_TYPE_BYTES,
        Timestamp = DPI_NATIVE_TYPE_TIMESTAMP,
        IntervalDs = DPI_NATIVE_TYPE_INTERVAL_DS,
        IntervalYm = DPI_NATIVE_TYPE_INTERVAL_YM,
        Lob = DPI_NATIVE_TYPE_LOB,
        Object = DPI_NATIVE_TYPE_OBJECT,
        Stmt = DPI_NATIVE_TYPE_STMT,
        Boolean = DPI_NATIVE_TYPE_BOOLEAN,
        RowId = DPI_NATIVE_TYPE_ROWID,
    }

    /// Oracle types corresponding to [ODPI-C dpiOracleTypeNum][]
    ///
    /// [ODPI-C dpiOracleTypeNum]: https://oracle.github.io/odpi/doc/enums/dpiOracleTypeNum.html
    #[derive(Debug, Clone, PartialEq)]
    #[repr(u32)]
    pub enum OracleType : dpiOracleTypeNum {
        Varchar = DPI_ORACLE_TYPE_VARCHAR,
        NVarchar = DPI_ORACLE_TYPE_NVARCHAR,
        Char = DPI_ORACLE_TYPE_CHAR,
        NChar = DPI_ORACLE_TYPE_NCHAR,
        RowId = DPI_ORACLE_TYPE_ROWID,
        Raw = DPI_ORACLE_TYPE_RAW,
        NativeFloat = DPI_ORACLE_TYPE_NATIVE_FLOAT,
        NativeDouble = DPI_ORACLE_TYPE_NATIVE_DOUBLE,
        NativeInt = DPI_ORACLE_TYPE_NATIVE_INT,
        Number = DPI_ORACLE_TYPE_NUMBER,
        Date = DPI_ORACLE_TYPE_DATE,
        Timestamp = DPI_ORACLE_TYPE_TIMESTAMP,
        TimestampTz = DPI_ORACLE_TYPE_TIMESTAMP_TZ,
        TimestampLtz = DPI_ORACLE_TYPE_TIMESTAMP_LTZ,
        IntervalDs = DPI_ORACLE_TYPE_INTERVAL_DS,
        IntervalYm = DPI_ORACLE_TYPE_INTERVAL_YM,
        CLob = DPI_ORACLE_TYPE_CLOB,
        NCLob = DPI_ORACLE_TYPE_NCLOB,
        BLob = DPI_ORACLE_TYPE_BLOB,
        BFile = DPI_ORACLE_TYPE_BFILE,
        Stmt = DPI_ORACLE_TYPE_STMT,
        Boolean = DPI_ORACLE_TYPE_BOOLEAN,
        Object = DPI_ORACLE_TYPE_OBJECT,
        LongVarchar = DPI_ORACLE_TYPE_LONG_VARCHAR,
        LongRaw = DPI_ORACLE_TYPE_LONG_RAW,
        NativeUint = DPI_ORACLE_TYPE_NATIVE_UINT,
    }
}

#[derive(Clone, Debug)]
pub struct DataTypeInfo {
    pub oracle_type: OracleType,
    pub default_native_type: NativeType,
    pub oci_type_code: u16,
    pub db_size_in_bytes: u32,
    pub client_size_in_bytes: u32,
    pub size_in_chars: u32,
    pub precision: i16,
    pub scale: i8,
    pub fs_precision: u8,
    pub object_type: Option<ObjectType>,
}

impl DataTypeInfo {
    pub fn new(ctxt: Context, info: &dpiDataTypeInfo) -> Result<DataTypeInfo> {
        Ok(DataTypeInfo {
            oracle_type: info.oracleTypeNum.try_into()?,
            default_native_type: info.defaultNativeTypeNum.try_into()?,
            oci_type_code: info.ociTypeCode,
            db_size_in_bytes: info.dbSizeInBytes,
            client_size_in_bytes: info.clientSizeInBytes,
            size_in_chars: info.sizeInChars,
            precision: info.precision,
            scale: info.scale,
            fs_precision: info.fsPrecision,
            object_type: if info.objectType.is_null() {
                None
            } else {
                Some(ObjectType::with_add_ref(ctxt, info.objectType)?)
            },
        })
    }
}

#[derive(Clone, Debug)]
pub struct ObjectTypeInfo {
    pub schema: String,
    pub name: String,
    pub is_collection: bool,
    pub element_type_info: Option<DataTypeInfo>,
    pub num_attributes: u16,
}

impl ObjectTypeInfo {
    pub fn new(ctxt: Context, info: &dpiObjectTypeInfo) -> Result<ObjectTypeInfo> {
        Ok(ObjectTypeInfo {
            schema: to_rust_str(info.schema, info.schemaLength),
            name: to_rust_str(info.name, info.nameLength),
            is_collection: info.isCollection != 0,
            element_type_info: if info.isCollection != 0 {
                Some(DataTypeInfo::new(ctxt, &info.elementTypeInfo)?)
            } else {
                None
            },
            num_attributes: info.numAttributes,
        })
    }
}

#[derive(Clone, Debug)]
pub struct ObjectAttrInfo {
    pub name: String,
    pub type_info: DataTypeInfo,
}

impl ObjectAttrInfo {
    pub fn new(ctxt: Context, info: &dpiObjectAttrInfo) -> Result<ObjectAttrInfo> {
        Ok(ObjectAttrInfo {
            name: to_rust_str(info.name, info.nameLength),
            type_info: DataTypeInfo::new(ctxt, &info.typeInfo)?,
        })
    }
}
