---
title: "TypeScript: Iterating over objects"
categories:
- TypeScript
--- 

There is rarely a head-scratcher in TypeScript as prominent as trying to access an object property via iterating through its keys. This is a pattern that's so common in JavaScript, yet TypeScript seems to through all the obstacles at you. This simple line:

```typescript
Object.keys(person).map(k => person[k])
```

has TypeScript throwing red squigglies at you and developers flipping tables. It's just not fun. There are several solutions to that. I tried to ["improve" `Object.keys` once](/typescript-better-object-keys/). It's a nice exercise on declaration merging but uh... I wouldn't do that too often. Also [Dan writes profoundly about this](https://effectivetypescript.com/2020/05/26/iterate-objects/). Annotating definitely is one solution.

But hey, let's look at the problem first.

## Why iterating over objects isn't so easy

Let's take a look at this function:

```typescript
type Person = {
  name: string,
  age: number
}

function printPerson(p: Person) {
  Object.keys(p).forEach((k) => {
      console.log(k, p[k]) // ERROR!!
  })
}
```

All we want is to print a `Person`'s fields by accessing them through their keys. TypeScript won't allow this. `Object.keys(p)` returns a `string[]`, which is too wide to allow accessing a very defined object shape `Person`. 

But why is that so? Isn't it obvious that we only access keys that are available? That's the whole point of using `Object.keys`!

Sure, but we are also able to pass objects that are subtypes of `Person`, which have more properties than defined in `Person`.

```typescript
const me = {
  name: "Stefan",
  age: 40,
  website: "https://fettblog.eu"
}

printPerson(me); // All good!
```

So, you might tell me that still `printPerson` should work correctly. It prints more properties, ok, but it doesn't break the code. It's still the keys of `p`, so every property should be accessible.

Sure, but what if you don't access `p`?

So, let's assume `Object.keys` gives you `(keyof Person)[]`. Just like [my 2 year old "fix" tries to do](/typescript-better-object-keys/). You can easily write something like this:

```typescript
function printPerson(p: Person) {
  const you: Person = {
    name: "Reader",
    age: NaN
  };

  Object.keys(p).forEach((k) => {
    console.log(k, you[k])
  })  
}

const me = {
  name: "Stefan",
  age: 40,
  website: "https://fettblog.eu"
}

printPerson(me);
```

If `Object.keys(p)` returns an array of type `keyof Person[]`, you will be able to access other objects of `Person`, too. This might not add up. In our example, we *just* print undefined. But what if you try to do something with those values. This will break at runtime.

TypeScript prevents you from scenarios like this. It's honest and says: Well, you think it might be `keyof Person`, but in reality, it can be so much more. 

Only type guards can help you:

```typescript
function isKey<T>(x: T, k: PropertyKey): k is keyof T {
  return k in x
}

function printPerson(p: Person) {
  Object.keys(p).forEach((k) => {
      if(isKey(p, k)) console.log(k, p[k]) // All fine!
  })
}
```

But... not so nice, isn't it?

## for-in loops

There's another way to iterate over objects:

```typescript
function printPerson(p: Person) {
  for (let k in p) {
    console.log(k, p[k]) // Error
  }
}
```

TypeScript gives you the same error: *Element implicitly has an 'any' type because expression of type 'string' can't be used to index type 'Person'.* For the same reason. You still can do something like this:


```typescript
function printPerson(p: Person) {
  const you: Person = {
    name: "Reader",
    age: NaN
  };

  for (let k in p) {
    console.log(k, you[k])
  } 
}

const me = {
  name: "Stefan",
  age: 40,
  website: "https://fettblog.eu"
}

printPerson(me);
```

And it will explode at runtime.

However, writing it like this gives you a little edge over the `Object.keys` version. TypeScript can be much more exact in this scenario if you add a generics:

```typescript
function printPerson<T extends Person>(p: T) {
  for (let k in p) {
    console.log(k, p[k]) // This works
  }
}
```

Instead of requiring `p` to be `Person` (and thus be compatible with all sub-types of `Person`), we add a new generic type parameter `T` that extends from `Person`. This means that all types that have been compatible to this function signature are still compatible, but the moment we use `p`, we are dealing with an explicit sub-type, not the broader super-type `Person`.

We substitute `T` for something that is compatible with Person, but where TypeScript knows that it's different enough to prevent you from errors. 

The code above works. `k` is of type `keyof T`. That's why we can access `p`, which is of type `T`. `T` being a sub-type of `Person`, that's just coincidence.

But we won't be able to do stuff that might break, like this:

```typescript
function printPerson<T extends Person>(p: T) {
  const you: Person = {
    name: "Reader",
    age: NaN
  }
  for (let k in p) {
    console.log(k, you[k]) // ERROR
  }
}
```

We can't access a `Person` with `keyof T`. They might be different. Beautiful!

And since `T` is a sub-type of `Person`, we still can assign properties:

```typescript
p.age = you.age
```

Great!

## Bottom line

TypeScript being very conservative about its types here is something that might seem odd at first but helps you in scenarios you wouldn't think of. I guess this is the part where JavaScript developers usually scream at the compiler and think they're "fighting" it, but hey, maybe TypeScript saved your butt. For situations where this gets annoying, TypeScript at least gives you ways to workaround.