use anyhow::anyhow;
use cosmetics_parser::*;
use pest::Parser;

#[test]
fn test_whitespace() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::WHITESPACE, " ")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), " ");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 1);

    let pair = Grammar::parse(Rule::WHITESPACE, "");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    Ok(())
}

#[test]
fn test_space() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::SPACE, "  ")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "  ");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 2);

    let pair = Grammar::parse(Rule::SPACE, "");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    Ok(())
}

#[test]
fn test_product() -> anyhow::Result<()> {
    let input = "*Product 1*: Face Cream \"Moisturizing\"
     *Skin Type*: Dry Skin
     *Ingredients*: Water, Glycerin, Hyaluronic Acid, Jojoba Oil
     *Rating*: 4.5
     *Price*: 299.99 UAH
     *User Ratings*: [5, 4, 5, 3, 4]
     *Recommendations*: Use in the morning and evening after cleansing the skin. Suitable for sensitive skin.
     *Reviews*:
     1.	\"This cream perfectly moisturizes my skin. It absorbs easily!\"
     2.	\"No excessive shine, perfect for autumn.\"
     *Availability*: true\n";

    let pair = Grammar::parse(Rule::product, &input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), input);
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 485);

    let pair = Grammar::parse(Rule::product, "*Product 1*: Face Cream \"Moisturizing\"
     *Skin Type*: Dry Skin
     *Ingredients*: Water, Glycerin, Hyaluronic Acid, Jojoba Oil
     *Price*: 299.99 UAH
     *User Ratings*: [5, 4, 5, 3, 4]
     *Recommendations*: Use in the morning and evening after cleansing the skin. Suitable for sensitive skin.
     *Reviews*:
     1.	\"This cream perfectly moisturizes my skin. It absorbs easily!\"
     2.	\"No excessive shine, perfect for autumn.\"
     *Availability*: true\n");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    let pair = Grammar::parse(Rule::product, "");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    Ok(())
}

#[test]
fn test_rating() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::rating, "*Rating*: 4.5\n")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "*Rating*: 4.5\n");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 14);

    let pair = Grammar::parse(Rule::rating, "*Rating*:\n");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    let pair = Grammar::parse(Rule::rating, "");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    Ok(())
}

#[test]
fn test_availability() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::availability, "*Availability*: true\n")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "*Availability*: true\n");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 21);

    let pair = Grammar::parse(Rule::availability, "*Availability*: нема\n");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    let pair = Grammar::parse(Rule::availability, "");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    Ok(())
}

#[test]
fn test_price() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::price, "*Price*: 299.99 UAH\n")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "*Price*: 299.99 UAH\n");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 20);

    let pair = Grammar::parse(Rule::price, "299.99 UAH\n");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    let pair = Grammar::parse(Rule::price, "");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    Ok(())
}

#[test]
fn test_user_ratings() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::user_ratings, "*User Ratings*: [5, 4, 5, 3, 4]\n")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "*User Ratings*: [5, 4, 5, 3, 4]\n");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 32);

    let pair = Grammar::parse(Rule::user_ratings, "*User Ratings*: 5, 4, 5");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    let pair = Grammar::parse(Rule::user_ratings, "");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    Ok(())
}

#[test]
fn test_product_name() -> anyhow::Result<()> {
    let pair = Grammar::parse(
        Rule::product_name,
        "*Product 1*: Face Cream \"Moisturizing\"\n",
    )?
    .next()
    .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "*Product 1*: Face Cream \"Moisturizing\"\n");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 39);

    let pair = Grammar::parse(
        Rule::product_name,
        "Product 1: Face Cream \"Moisturizing\"\n",
    );
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    let pair = Grammar::parse(Rule::product_name, "");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    Ok(())
}

#[test]
fn test_recommendations() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::recommendations, "*Recommendations*: Use in the morning and evening after cleansing the skin. Suitable for sensitive skin.\n")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "*Recommendations*: Use in the morning and evening after cleansing the skin. Suitable for sensitive skin.\n");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 105);

    let pair = Grammar::parse(Rule::recommendations, "*Recommendations*:\n");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    let pair = Grammar::parse(Rule::recommendations, "");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    Ok(())
}

#[test]
fn test_ingredients() -> anyhow::Result<()> {
    let pair = Grammar::parse(
        Rule::ingredients,
        "*Ingredients*: Water, Glycerin, Hyaluronic Acid, Jojoba Oil\n",
    )?
    .next()
    .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(
        pair.as_str(),
        "*Ingredients*: Water, Glycerin, Hyaluronic Acid, Jojoba Oil\n"
    );
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 60);

    let pair = Grammar::parse(
        Rule::ingredients,
        "Water, Glycerin, Hyaluronic Acid, Jojoba Oil\n",
    );
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    let pair = Grammar::parse(Rule::ingredients, "");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    Ok(())
}

#[test]
fn test_reviews() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::reviews, "*Reviews*:\n 1.	\"This cream perfectly moisturizes my skin. It absorbs easily!\" 2.	\"No excessive shine, perfect for autumn.\"\n")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(),"*Reviews*:\n 1.	\"This cream perfectly moisturizes my skin. It absorbs easily!\" 2.	\"No excessive shine, perfect for autumn.\"\n");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 123);

    let pair = Grammar::parse(
        Rule::reviews,
        "*Review*:\n 1.	\"This cream perfectly moisturizes my skin. It absorbs easily!\"\n",
    );
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    let pair = Grammar::parse(Rule::reviews, "");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    Ok(())
}

#[test]
fn test_skin_type() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::skin_type, "*Skin Type*: Dry Skin\n")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "*Skin Type*: Dry Skin\n");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 22);

    let pair = Grammar::parse(Rule::skin_type, "*Skin Type*:\n");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    let pair = Grammar::parse(Rule::skin_type, "");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    Ok(())
}

#[test]
fn test_number() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::number, "299.99")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "299.99");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 6);

    let pair = Grammar::parse(Rule::number, "299")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "299");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 3);

    let pair = Grammar::parse(Rule::number, "-273.15")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "-273.15");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 7);

    let pair = Grammar::parse(Rule::number, "-273")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "-273");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 4);

    let pair = Grammar::parse(Rule::number, "");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    Ok(())
}

#[test]
fn test_number_list() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::number_list, "[5, 4, 5, 3, 4]")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "[5, 4, 5, 3, 4]");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 15);

    let pair = Grammar::parse(Rule::number_list, "5 4 5 3 4");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    let pair = Grammar::parse(Rule::number_list, "");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    Ok(())
}

#[test]
fn test_single_review() -> anyhow::Result<()> {
    let pair = Grammar::parse(
        Rule::review,
        "1.\"This cream perfectly moisturizes my skin. It absorbs easily!\"\n",
    )?
    .next()
    .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(
        pair.as_str(),
        "1.\"This cream perfectly moisturizes my skin. It absorbs easily!\"\n"
    );
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 65);

    let pair = Grammar::parse(
        Rule::review,
        "\"This cream perfectly moisturizes my skin. It absorbs easily!\"\n",
    );
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    let pair = Grammar::parse(Rule::review, "");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    Ok(())
}

#[test]
fn test_currency() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::currency, "UAH")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "UAH");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 3);

    let pair = Grammar::parse(Rule::currency, "EUR")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "EUR");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 3);

    let pair = Grammar::parse(Rule::currency, "USD")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "USD");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 3);

    let pair = Grammar::parse(Rule::currency, "IDR");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    let pair = Grammar::parse(Rule::currency, "");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    Ok(())
}

#[test]
fn test_any_text() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::any_text, "gbehfhr behjf 45 bhcjbhcb jhjfhjrb\n")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "gbehfhr behjf 45 bhcjbhcb jhjfhjrb\n");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 35);

    let pair = Grammar::parse(Rule::any_text, "gbehfhr behjf 45 bhcjbhcb jhjfhjrb");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    let pair = Grammar::parse(Rule::any_text, "");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    Ok(())
}

#[test]
fn test_bool() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::boolean, "true")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), "true");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 4);

    let pair = Grammar::parse(Rule::boolean, "gbehfhr");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    let pair = Grammar::parse(Rule::boolean, "");
    assert!(pair.is_err(), "Expected error but got {:?}", pair);

    Ok(())
}
