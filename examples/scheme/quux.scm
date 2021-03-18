;; Constructs a new list by applying `f` to each element of `xs`.
(define (map f xs)
  (if (null? xs)
      '()
      (cons (f (car xs))
            (map f (cdr xs)))))

;; Tests if `n` is even
;; Note the use of mutual recursion via `odd`.
(define (even? n)
  (if (= n 0)
      #t
      (odd? (- n 1))))

(define (odd? n)
  (if (= n 0)
      #f
      (even? (- n 1))))

;; Constructs a new symbol by adding ".<i>" to `sym`
(define (add-index sym i)
  (string->symbol
    (string-append (symbol->string sym)
                   "."
                   (number->string i))))

(define quux (add-index "my-symbol" 42))