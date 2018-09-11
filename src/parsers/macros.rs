// -------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! parsec {
    ($a:lifetime, $e:expr) => { Parsec::<$a>(Box::new($e)) };
    ($e:expr) => { parsec(Box::new($e)) };
}

// -------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! lazy {
    ($e:expr) => { lazy(|| $e) };
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
    (_variables $a:ident <- ($e:expr) if ($cond:expr) $($r:tt)+) => {
        foreach!(_variables $a <- ($e.filter(move |&$a| $cond)) $($r)+)
    };
    (_variables $a:ident <- ($e:expr) yield $result:expr) => {
        $e.map(move |$a| $result)
    };
    (_variables $a:ident <- ($e:expr) $($r:tt)+) => {
        $e.flat_map(move |$a| foreach!(_variables $($r)+))
    };
    (_variables ($e:expr) yield $result:expr) => {
        $e.map(move |_| $result)
    };
    (_variables ($e:expr) $($r:tt)+) => {
        $e.then_right(foreach!(_variables $($r)+))
    };
    ($($r:tt)+) => {
        foreach!(_variables $($r)+)
    }
}

// -------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! foreach2 {
    (_variables ($($v:ident)+) _body $a:ident <- ($e:expr) if ($cond:expr) $($r:tt)+) => {
        foreach2!(_variables ($($v:ident)+) _body $a <- ($e.filter(move |&$a| $cond)) $($r)+)
    };
    (_variables ($($v:ident)+) _body $a:ident <- ($e:expr) yield $result:expr) => {
        $e.map(move |$a| $result)
    };
    (_variables ($($v:ident)+) _body $a:ident <- ($e:expr) $($r:tt)+) => {
        $e.then(foreach2!(_variables ($($v:ident)+ $a)  _body $($r)+))
    };
    (_variables ($($v:ident)+) _body ($e:expr) yield $result:expr) => {
        $e.map(move |_| $result)
    };
    (_variables ($($v:ident)+) _body ($e:expr) $($r:tt)+) => {
        $e.then_right(foreach2!(_variables ($($v)+) _body $($r)+))
    };
    ($($r:tt)+) => {
        foreach2!(_variables () _body $($r)+)
    }
}

// -------------------------------------------------------------------------------------------------
