# CosmeticsParser

## Overview
`CosmeticsParser` is a Rust-based parser designed to extract information from a cosmetics catalog written in a human-readable markdown format. The input consists of product descriptions that include details such as product name, skin type, ingredients, ratings, price, user reviews, and availability.

The parser reads these product descriptions and converts them into a structured data format, which can be used for further processing, analysis, or presentation in an online cosmetics store application.

## Parsing Process
The parser reads a markdown-like format with structured information for each product. Each product contains the following fields:

1. **Product Name**: The name of the product.
2. **Skin Type**: The type of skin the product is designed for (e.g., dry, oily).
3. **Ingredients**: The ingredients used in the product.
4. **Rating**: The overall rating of the product.
5. **Price**: The price of the product.
6. **User Ratings**: A list of user ratings.
7. **Recommendations**: Instructions or recommendations for using the product.
8. **Reviews**: User-submitted feedback.
9. **Availability**: A boolean value indicating whether the product is in stock.

### Grammar
The parser uses the Pest library to process the input format. The grammar rule defined in `grammar.pest` handles product descriptions and processes fields such as numbers, strings, and lists (e.g., user ratings).

### How It Works And Where To Use
The input is processed line by line, and the parser extracts relevant data from each field. After parsing, a `CosmeticsCatalog` object is created to hold the parsed products. This catalog can then be used for further processing or display in a frontend application.

### Example Input
```markdown
*Product 1*: Face Cream "Moisturizing"
*Skin Type*: Dry Skin
*Ingredients*: Water, Glycerin, Hyaluronic Acid, Jojoba Oil
*Rating*: 4.5
*Price*: 299.99 UAH
*User Ratings*: [5, 4, 5, 3, 4]
*Recommendations*: Use in the morning and evening after cleansing the skin. Suitable for sensitive skin.
*Reviews*:
1.	"This cream perfectly moisturizes my skin. It absorbs easily!"
2.	"No excessive shine, perfect for autumn."
*Availability*: true
```

### Example Output
```json
{
  "name": "Face Cream 'Moisturizing'",
  "skin_type": "Dry Skin",
  "ingredients": ["water", "glycerin", "hyaluronic acid", "jojoba oil"],
  "rating": 4.5,
  "price": 299.99,
  "user_ratings": [5, 4, 5, 3, 4],
  "recommendations": "Use in the morning and evening after cleansing your skin. Suitable for sensitive skin.",
  "reviews": [
    "This cream moisturizes my skin well. It absorbs easily!",
    "No excess shine, perfect for autumn."
  ],
  "availability": true
}
```

