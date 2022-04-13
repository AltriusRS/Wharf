# Syntax Specification
- [Tags](#tags)
- [Basic Usage](#basic usage)
- [Multiline Comments](#multiline comments)


## Tags

| Tag      | Description                                            | Format                              |
|:---------|:-------------------------------------------------------|:------------------------------------|
| @Name    | Describes the name of the object                       | @Name: \<name>                      |
| @Type    | Describes the type of the object                       | @Type: \<type>                      |
| @Example | Provides an example of how to use the described object | @Example: myFunction("sample data") |
| @Param   | describes a parameter of a method                      | @Param: \<type> \<name> \<example>  |

## Basic Usage
Wharf uses your programming languages default comment syntax, 
followed by a # (referred to as the `WharfID`) to identify comments which should be processed.
Any comment which is followed by the `WharfID` will be processed as a Wharf compatible comment.


Basic comment:

Sample Language: <a style="color:#ff0049">Javascript</a>
```js
//# @Name: SomeClass
//# @Example: new SomeClass(someValue)
//# additional description information
class SomeClass {
    constructor(value) {
        this.value = value;
    }
}
```
#### Component Breakdown:
<b>@Name</b> - The name of the described object.<br />
<b>@Example</b> - An example usage of the object.<br />



Sample Language: <a style="color:#ff0049">Javascript</a>
```js
//# @Name: someConstant
//# @Type: SomeClass
//# Some additional description information
const someConstant = new SomeClass(someValue);
```
#### Component Breakdown:
<b>@Name</b> - the name of the described object<br />
<b>@Type</b> - the type of the object<br />



## Multiline Comments

Sample Language: <a style="color:#ff0049">Javascript</a>
```js
/*#
  @Name: someConstant
  @Example: new SomeClass(someValue)
  additional description information
 */
class SomeClass {
    constructor(value) {
        this.value = value;
    }
}
```
#### Component Breakdown:
<b>@Name</b> - The name of the described object.<br />
<b>@Example</b> - An example usage of the object.<br />



Sample Language: <a style="color:#ff0049">Javascript</a>
```js
/*#
  @Name: someConstant
  @Type: SomeClass
  Some additional description information
 */
const someConstant = new SomeClass(someValue);
```
#### Component Breakdown:
<b>@Name</b> - the name of the described object<br />
<b>@Type</b> - the type of the object<br />