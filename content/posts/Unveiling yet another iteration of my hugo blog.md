+++
title = "Unveiling yet another iteration of my hugo blog"
slug = "unveiling-yet-another-iteration-of-my-hugo-blog"
author = "Timo Sarkar"
date = 2020-01-27T17:58:23-08:00
categories = ["announcement"]
tags = ["announcement", "my blog"]
katex = true
draft = false
+++

> Do not mind this content here. I am trying out some katex equations

```clojure
(print "hello, world")
```

$$ k(\varphi) = \varphi - 1 $$


$$ n(\varphi) = z - \varphi $$



$$ n(\varphi, z) = z - \varphi $$


$$ F(z) = Fib(z) = \sum_{\varphi=1}^{z_{end}\ =\  round(\frac{z}{2})} {_{n(\varphi, z)\  =\  z - \varphi}}C _{k(\varphi)\  =\  \varphi - 1} $$

$$ F(z) = \sum_{\varphi=1}^{round(\frac{z}{2})}{_{z - \varphi}}C _{\varphi - 1} $$



$$ F(9) = \sum_{\varphi=1}^{5}{_{9 - \varphi}}C _{\varphi - 1} $$

$$
=\ _{9 - 1}C _{1 - 1} +
   \ _{9 - 2}C _{2 - 1} +
   \ _{9 - 3}C _{3 - 1} +
   \ _{9 - 4}C _{4 - 1} +
   \ _{9 - 5}C _{5 - 1}
$$

$$
=\ _{8}C _{0} +
\ _{7}C _{1} +
\ _{6}C _{2} +
\ _{5}C _{3} +
\ _{4}C _{4}
$$

$$=\ 1 + 7 + 15 + 10 + 1$$

$$=\ 34$$

