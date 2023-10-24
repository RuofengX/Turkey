use std::borrow::Cow;

use pyo3::{prelude::*, exceptions::{PyKeyError, PyIOError}};

use retable::Database;
use bson::Bson;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "turkey")]
fn extension(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<TurkeyMap>()?;
    Ok(())
}

#[pyclass]
pub struct TurkeyMap{
    _db: Database,
}

#[pymethods]
impl TurkeyMap{
    #[new]
    pub fn new() -> Self{
        Self { _db: Database::new() }
    }

    /// 根据实体ID、属性名、属性值Bson插入
    fn insert(&mut self, ent_id: u32, prop_name: String, prop: BsonBytes) -> PyResult<()>{
        // 尝试转换属性值为Bson
        let prop = bson::from_slice::<Bson>(&prop);
        if prop.is_err(){
            return  Err(PyIOError::new_err("无法将属性值转换为Bson结构体"));
        }
        // 尝试插入
        if let Ok(_) = self._db.insert(
            ent_id, 
            prop_name, 
            prop.unwrap()){
                Ok(())
            }else {
                return Err(PyKeyError::new_err("已存在相同的键"))
            }
    }

    /// 根据实体ID获取属性值
    pub fn get_by_id(&self, ent_id: u32) -> Vec<ReturnBsonBytes>{
        self._db.get_props(ent_id)
        .into_iter()
        .map(|atom|bson::to_vec(atom).unwrap())
        .map(|b|Cow::Owned(b))
        .collect::<Vec<ReturnBsonBytes>>()

    }
    /// 根据属性名获取属性值
    pub fn get_by_prop(&self, prop_name: String) -> Vec<ReturnBsonBytes>{
        self._db.get_entities(prop_name)
        .into_iter()
        .map(|atom|bson::to_vec(atom).unwrap())
        .map(|b|Cow::Owned(b))
        .collect::<Vec<ReturnBsonBytes>>()
    }

    /// 根据实体ID和属性名查找唯一值
    pub fn get(&self, ent_id: u32, prop_name: String) -> Option<ReturnBsonBytes>{
        if let Some(atom) = self._db.get(ent_id, prop_name){
            Some(Cow::Owned(bson::to_vec(atom).unwrap()))
        } else {
            None
        }
    }

    /// 根据实体ID和属性名更新属性值
    pub fn update(&mut self, ent_id: u32, prop_name: String, prop: BsonBytes) -> PyResult<()>{
        if let Ok(prop) = bson::from_slice(&prop){
            if let Some(_) = self._db.update(ent_id, prop_name, prop){
                Ok(())
            } else {
                Err(PyKeyError::new_err("查询键不存在"))
            }
        } else {
            return Err(PyIOError::new_err("无法将属性值转换为Bson结构体"))
        }
    }
}

/// Bson字符串
type BsonBytes = Vec<u8>;
type ReturnBsonBytes = Cow<'static, [u8]>;
