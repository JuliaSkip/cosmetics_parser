# cosmetics_parser

## Link
https://crates.io/crates/cosmetics_parser

https://docs.rs/cosmetics_parser/0.1.0/cosmetics_parser/

## Overview
`cosmetics_parser` is a Rust-based parser designed to extract information from a cosmetics catalog written in a human-readable markdown format. The input consists of product descriptions that include details such as product name, skin type, ingredients, ratings, price, user reviews, and availability.

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

- WHITESPACE  
A whitespace character, which can be a space or a tab

```
WHITESPACE = { " " | "\t" }
```

- SPACE  
One or more whitespace characters

```
SPACE = { WHITESPACE+ }
```

- products  
A list of one or more products, separated by either a newline or space

```
products = { (product ~ (NEWLINE | SPACE))* }
```

- product  
A single product entry, which consists of various fields like product name, skin type, ingredients, etc

```
product = { product_name ~ skin_type ~ ingredients ~ rating ~ price ~ user_ratings ~ recommendations ~ reviews ~ availability }
```

- rating  
The rating of a product, denoted by the "*Rating*:" label, followed by an optional space, a number, and a newline

```
rating = { "*Rating*:" ~ SPACE? ~ number ~ NEWLINE }
```

- availability  
The availability of a product, denoted by the "*Availability*:" label, followed by an optional space and a boolean value (true/false)

```
availability = { "*Availability*:" ~ SPACE? ~ boolean ~ NEWLINE }
```

- price  
The price of a product, denoted by the "*Price*:" label, followed by an optional space, a number, an optional space, an optional currency, and a newline

```
price = { "*Price*:" ~ SPACE? ~ number ~ SPACE? ~ currency? ~ NEWLINE }
```

- user_ratings  
The user ratings of a product, denoted by the "*User Ratings*:" label, followed by a list of numbers and a newline

```
user_ratings = { "*User Ratings*:" ~ SPACE? ~ number_list ~ NEWLINE }
```

- product_name  
The product name, denoted by the "*Product" label, followed by a number (product identifier), a colon, and the product name text

```
product_name = { "*Product " ~ (ASCII_DIGIT+) ~ "*:" ~ any_text }
```

- recommendations  
Recommendations for the product, denoted by the "*Recommendations*:" label followed by any text describing the recommendations

```
recommendations = { "*Recommendations*:" ~ any_text }
```

- ingredients  
Ingredients of the product, denoted by the "*Ingredients*:" label followed by any text describing the ingredients

```
ingredients = { "*Ingredients*:" ~ any_text }
```

- reviews  
Reviews of the product, denoted by the "*Reviews*:" label followed by optional space and one or more reviews

```
reviews = { "*Reviews*:" ~ SPACE? ~ (review)* }
```

- skin_type  
The skin type of the product, denoted by the "*Skin Type*:" label followed by text describing the skin type

```
skin_type = { "*Skin Type*:" ~ any_text }
```

- number  
A number, which can be an integer or a floating-point number, optionally starting with a negative sign

```
number = { ("-"? ~ ASCII_DIGIT+) ~ (("." ~ ASCII_DIGIT+)?) }
```

- number_list  
A list of numbers enclosed in square brackets, separated by commas

```
number_list = { "[" ~ number ~ ("," ~ SPACE? ~ number)* ~ "]" }
```

- review  
A product review, which consists of a number (rating) followed by a period and some text

```
review = { NEWLINE? ~ number ~ "." ~ any_text }
```

- currency  
A currency symbol, which can be "UAH", "EUR", or "USD"

```
currency = { "UAH" | "EUR" | "USD" }
```

- any_text  
Any text, which can be any sequence of characters (except newline), optionally preceded by whitespace

```
any_text = { SPACE? ~ (!NEWLINE ~ ANY)+ ~ NEWLINE }
```

- boolean  
A boolean value, which can be either "true" or "false"

```
boolean = { ("true" | "false") }
```


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
*Availability*: true
```

### Example Output
```json
{
    "product_name": "Face Cream \"Moisturizing\"",
    "skin_type": "Dry Skin",
    "ingredients": "Water, Glycerin, Hyaluronic Acid, Jojoba Oil",
    "rating": 4.5,
    "price": 299.99,
    "user_ratings": [
      5.0,
      4.0,
      5.0,
      3.0,
      4.0
    ],
    "recommendations": "Use in the morning and evening after cleansing the skin. Suitable for sensitive skin.",
    "reviews": [
      "1. \"This cream perfectly moisturizes my skin. It absorbs easily!\""
    ],
    "availability": true
  }
```

