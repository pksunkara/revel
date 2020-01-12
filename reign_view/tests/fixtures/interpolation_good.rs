write!(f, "{}", "<div")? ;
write!(f, ">")? ;
write!(f, "{}", "\n  ")? ;
write!(f, "{}", "<span")? ;
write!(f, ">")? ;
write!(f, "{}{}{}{}{}{}{}{}", "Ident ", self.title, " and ", "user", b"user", b'u', 'u', 10)? ;
write!(f, "{}", "</span>")? ;
write!(f, "{}", "\n  ")? ;
write!(f, "{}", "<span")? ;
write!(f, ">")? ;
write!(f, "{}{}{}", "Escaped ", "\\{{", " title }}")? ;
write!(f, "{}", "</span>")? ;
write!(f, "{}", "\n  ")? ;
write!(f, "{}", "<span")? ;
write!(f, ">")? ;
write!(f, "{}{}{}", "With &gt; and &lt; inside ", "<", ">")? ;
write!(f, "{}", "</span>")? ;
write!(f, "{}", "\n  ")? ;
write!(f, "{}", "<span")? ;
write!(f, ">")? ;
write!(f, "{}{}", "Array ", [self.a, self.b])? ;
write!(f, "{}", "</span>")? ;
write!(f, "{}", "\n  ")? ;
write!(f, "{}", "<span")? ;
write!(f, ">")? ;
write!(f, "{}{}", "Binary ", self.a + self.b)? ;
write!(f, "{}", "</span>")? ;
write!(f, "{}", "\n  ")? ;
write!(f, "{}", "<span")? ;
write!(f, ">")? ;
write!(f, "{}{}", "Call ", self.a(self.b, self.c))? ;
write!(f, "{}", "</span>")? ;
write!(f, "{}", "\n  ")? ;
write!(f, "{}", "<span")? ;
write!(f, ">")? ;
write!(f, "{}{}", "Cast ", self.a as i32)? ;
write!(f, "{}", "</span>")? ;
write!(f, "{}", "\n  ")? ;
write!(f, "{}", "<span")? ;
write!(f, ">")? ;
write!(f, "{}{}{}{}", "Field ", self.a.b, self.a.0, self.a.b.c)? ;
write!(f, "{}", "</span>")? ;
write!(f, "{}", "\n  ")? ;
write!(f, "{}", "<span")? ;
write!(f, ">")? ;
write!(f, "{}{}", "Index ", self.a[self.b])? ;
write!(f, "{}", "</span>")? ;
write!(f, "{}", "\n  ")? ;
write!(f, "{}", "<span")? ;
write!(f, ">")? ;
write!(f, "{}{}", "Method Call ", self.x.y:: <T>(self.a, self.b))? ;
write!(f, "{}", "</span>")? ;
write!(f, "{}", "\n  ")? ;
write!(f, "{}", "<span")? ;
write!(f, ">")? ;
write!(f, "{}{}", "Paren ", (self.a + self.b))? ;
write!(f, "{}", "</span>")? ;
write!(f, "{}", "\n  ")? ;
write!(f, "{}", "<span")? ;
write!(f, ">")? ;
write!(f, "{}{}", "Range ", self.a..self.b)? ;
write!(f, "{}", "</span>")? ;
write!(f, "{}", "\n  ")? ;
write!(f, "{}", "<span")? ;
write!(f, ">")? ;
write!(f, "{}{}", "Repeat ", [self.a; self.b])? ;
write!(f, "{}", "</span>")? ;
write!(f, "{}", "\n  ")? ;
write!(f, "{}", "<span")? ;
write!(f, ">")? ;
write!(f, "{}{}", "Struct ", A { a: self.a, b: self.b, ..self.c })? ;
write!(f, "{}", "</span>")? ;
write!(f, "{}", "\n  ")? ;
write!(f, "{}", "<span")? ;
write!(f, ">")? ;
write!(f, "{}{}", "Tuple ", (self.a, self.b))? ;
write!(f, "{}", "</span>")? ;
write!(f, "{}", "\n  ")? ;
write!(f, "{}", "<span")? ;
write!(f, ">")? ;
write!(f, "{}{}", "Type ", self.a: i32)? ;
write!(f, "{}", "</span>")? ;
write!(f, "{}", "\n  ")? ;
write!(f, "{}", "<span")? ;
write!(f, ">")? ;
write!(f, "{}{}", "Unary ", !self.a)? ;
write!(f, "{}", "</span>")? ;
write!(f, "{}", "\n  ")? ;
write!(f, "{}", "<span")? ;
write!(f, ">")? ;
write!(f, "{}{}", "Reference ", &self.a)? ;
write!(f, "{}", "</span>")? ;
write!(f, "{}", "\n")? ;
write!(f, "{}", "</div>")? ;
