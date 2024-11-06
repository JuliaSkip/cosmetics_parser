use pest::Parser;
use anyhow::anyhow;
use cosmetics_parser::*;


#[test]
fn test_product_name() -> anyhow::Result<()> {
    /// Тест на коректну назву продукту
    let pair = Grammar::parse(Rule::product_name, "*Product 1*: Face Cream \"Moisturizing\"\n")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    
    assert_eq!(pair.as_str(), "*Product 1*: Face Cream \"Moisturizing\"\n");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 39);

    /// Тест на некоректну назву продукту(без *)
    let pair = Grammar::parse(Rule::product_name, "Product 1: Face Cream \"Moisturizing\"\n");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    /// Тест на порожній рядок
    let pair = Grammar::parse(Rule::product_name, "");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    Ok(())
}

#[test]
fn test_user_ratings() -> anyhow::Result<()> {
    /// Тест коректного формату рейтингу
    let pair = Grammar::parse(Rule::user_ratings, "*User Ratings*: [5, 4, 5, 3, 4]\n")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    
    assert_eq!(pair.as_str(), "*User Ratings*: [5, 4, 5, 3, 4]\n");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 32);
    
    /// Тест некоректного формату рейтингу(список чисел без [])
    let pair = Grammar::parse(Rule::user_ratings, "*User Ratings*: 5, 4, 5");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    /// Тест на порожній рядок
    let pair = Grammar::parse(Rule::user_ratings, "");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    Ok(())
}

#[test]
fn test_number_list() -> anyhow::Result<()> {
    /// Тест коректного формату списку чисел
    let pair = Grammar::parse(Rule::number_list, "[5, 4, 5, 3, 4]")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    
    assert_eq!(pair.as_str(), "[5, 4, 5, 3, 4]");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 15);

    /// Тест некоректного формату списку чисел(без дужок [])
    let pair = Grammar::parse(Rule::number_list, "5 4 5 3 4");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    /// Тест на порожній рядок   
    let pair = Grammar::parse(Rule::number_list, "");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    Ok(())
}

#[test]
fn test_number() -> anyhow::Result<()> {
    /// Тест коректного формату числа(дріб)
    let pair = Grammar::parse(Rule::number, "299.99")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    
    assert_eq!(pair.as_str(), "299.99");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 6);

    /// Тест коректного формату числа(ціле число)
    let pair = Grammar::parse(Rule::number, "299")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    
    assert_eq!(pair.as_str(), "299");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 3);

    /// Тест відʼємного числа(дріб)
    let pair = Grammar::parse(Rule::number, "-273.15")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    
    assert_eq!(pair.as_str(), "-273.15");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 7);

    /// Тест відʼємного числа(ціле число)
    let pair = Grammar::parse(Rule::number, "-273")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    
    assert_eq!(pair.as_str(), "-273");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 4);


     /// Тест на порожній рядок   
     let pair = Grammar::parse(Rule::number, "");
     assert!(pair.is_err(), "Expected error but got {:?}", pair);

    Ok(())
}