// -------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! parsec {
    ($a:lifetime, $e:expr) => {
        Parsec::<$a>(Box::new($e))
    };
    ($e:expr) => {
        parsec(Box::new($e))
    };
}

// -------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! lazy {
    ($e:expr) => {
        lazy(|| $e)
    };
}

// -------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! seq {
    (* ($($r:tt)+))          => { seq!($($r)+).optrep()       };
    (($l:expr) <~ $($r:tt)+) => { $l.then_left(seq!($($r)+))  };
    (($l:expr) ~> $($r:tt)+) => { $l.then_right(seq!($($r)+)) };
    (($l:expr) ~  $($r:tt)+) => { $l.then(seq!($($r)+))       };
    (($l:expr) >> $r:expr)   => { $l.fmap(Box::new($r))       };
    (($l:expr))              => { $l                          };
}

// -------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! cases {
    (($l:expr) |  $($r:tt)+) => { $l.or(cases!($($r)+))       };
    (($l:expr) >> $r:expr)   => { $l.fmap(Box::new($r))       };
    ($l:expr)                => { $l                          };
}

// -------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! foreach {
    (_internal $a:ident <- ($e:expr) if ($cond:expr) $($r:tt)+) => {
        foreach!(_internal $a <- ($e.filter(move |&$a| $cond)) $($r)+)
    };
    (_internal $a:ident <- ($e:expr) yield $result:expr) => {
        $e.map(move |$a| $result)
    };
    (_internal $a:ident <- ($e:expr) $($r:tt)+) => {
        $e.flat_map(move |$a| foreach!(_internal $($r)+))
    };
    (_internal ($e:expr) yield $result:expr) => {
        $e.map(move |_| $result)
    };
    (_internal ($e:expr) $($r:tt)+) => {
        $e.then_right(foreach!(_internal $($r)+))
    };
    ($($r:tt)+) => {
        foreach!(_internal $($r)+)
    }
}

// -------------------------------------------------------------------------------------------------
