const Shape = {
  description: 'Unknown shape'
};

const fromPrototype = function (prototype, object) {
  const newObject = Object.create(prototype)
  for (const prop in object) {
    if (object.hasOwnProperty(prop)) {
      newObject[prop] = object[prop]
    }
  }
  return newObject
}

const ShapeFactory = {
  getShape: function (type) {
    if (type == 'circle') {
      return fromPrototype(Shape, {
        description: 'Circle shape'
      });
    } else if (type == 'square') {
      return fromPrototype(Shape, {
        description: 'Square shape'
      });
    }
  }

}

const shapeFactory = Object.create(ShapeFactory);
const circle = shapeFactory.getShape('circle');
const square = shapeFactory.getShape('square');

console.log(circle)
console.log(square)