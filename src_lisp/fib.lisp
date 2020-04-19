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
                inc    (lambda () ; our incrementer, which will always put the main value into b
                           (do (def! t (deref first)) (reset! first (deref second)) (reset! second (+ t (deref first))))
                       )
                step (lambda ()
                        (if (> n (deref count))
                            (do (inc) (swap! count (lambda (c) (+ c 1))) (step))
                            (deref second)
                        )
                     )
            )

            (step)
        )
    )

)