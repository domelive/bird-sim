//! A simple 2D vector struct with basic arithmetic operations and a dot product method.

/// A simple 2D vector struct with basic arithmetic operations and a dot product method.
#[derive(Debug, Clone, Copy)]
pub struct Vector2<T> {
    /// The x-coordinate of the vector.
    pub x: T,
    /// The y-coordinate of the vector.
    pub y: T,
}

/// Implement addition for `Vector2`, allowing you to add two vectors together.
impl<T> std::ops::Add for Vector2<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Vector2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// Implement subtraction for `Vector2`, allowing you to subtract one vector from another.
impl<T> std::ops::Sub for Vector2<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Vector2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// Implement element-wise multiplication for `Vector2`, allowing you to multiply two vectors together.
impl<T> std::ops::Mul for Vector2<T>
where
    T: std::ops::Mul<Output = T>,
{
    type Output = Vector2<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

/// Implement scalar multiplication for `Vector2`, allowing you to multiply a vector by a scalar value.
impl<T> std::ops::Mul<T> for Vector2<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Vector2<T>;

    fn mul(self, scalar: T) -> Self::Output {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

/// Implement a dot product method for `Vector2`, allowing you to calculate the dot product of two vectors.
impl<T> Vector2<T>
where
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy,
{
    pub fn dot(self, other: Self) -> T {
        (self.x * other.x) + (self.y * other.y)
    }
}

/// Implement a constructor for `Vector2` to create a new vector with specified x and y values.
impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
