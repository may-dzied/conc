{
    {  } into_vec u64_0
    u64_2 take
    {
        clone
        u64_2 take swap push
        swap u64_1 +
    } repeat
    delete
} "range" global_bind

{
    u64_0 insert
} "push" global_bind

{
    u64_0 remove
} "pop" global_bind

{
    "FILTER_TEST" scoped_bind
    {  } into_vec
    "FILTER_RESULT" scoped_bind
    {
        clone "FILTER_TEST" scoped_get call
        { "FILTER_RESULT" scoped_get swap push "FILTER_RESULT" scoped_bind u64_0 } if
        delete
    } for
    "FILTER_RESULT" scoped_get
} "filter" global_bind
