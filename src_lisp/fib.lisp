(def! slowfibn
    (lambda (n)
        (if (< n 3)
            1
            (+ (slowfibn (- n 1)) (slowfibn (- n 2)))
        )

    )
)

(def! fastfibn
    (lambda (n)
        (let
            (
                first  (atom 1)
                second (atom 1)
                count  (atom 2)
                count+ (lambda () (swap! count (lambda (c) (+ c 1))))
                inc    (lambda () ; our incrementer, which will always put the main value into b
                           (do (def! temp (deref first)) (reset! first (deref second)) (reset! second (+ temp (deref first))))
                       )
                step (lambda ()
                        (if (> n (deref count))
                            (do (inc) (count+) (step))
                            (deref second)
                        )
                     )
            )

            (step)
        )
    )

)


(def! fib (lambda (n)
    (let
        (fib-tree (lambda (n small large)
             (if (< n 3)
                large
                (fib-tree (- n 1) large (+ small large)))))

        (fib-tree n 1 1))))
