(def! fibn
    (lambda (n)
        (if (< n 3)
            1
            (+ (fibn (- n 1)) (fibn (- n 2)))
        )

    )
)