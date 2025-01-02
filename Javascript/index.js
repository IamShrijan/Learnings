// // console.log('HelloWorld');
// // let name='Mosh';
// // let firstName,LastName;
// // console.log(name);

// //Constants
// const interestRate = 0.3;
// //interestRate=1; //Will error out
// console.log(interestRate);

// //Datatypes
// let name = 'Shrijan';
// let age = 26;
// let isApproved = true;
// let firstName = null;
// let job = undefined;

// //Javascript is a dynamic language. datatype is determined at runtime. 

// //Reference types - objects 
// let person ={
//     name: 'Mosh',
//     age:30
// };
// //Dot Notation
// person.name = 'Shrijan'
// console.log(person.name);
// //Bracket notation 
// person['name'] = 'shlok';
// console.log(person['name']);
// //clever use case of bracket notaion 
// selection = 'name';
// person[selection]= 'puja';
// console.log(person[selection])

let selectedColors = ["red","blue"];
selectedColors[3]= 2;
console.log(selectedColors);
console.log(selectedColors.length);

function square(number){
    return number*number;
}
console.log(square(2));