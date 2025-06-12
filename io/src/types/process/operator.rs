//! https://github.com/apache/tinkerpop/blob/master/gremlin-core/src/main/java/org/apache/tinkerpop/gremlin/process/traversal/Operator.java

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Operator {
    Sum,
    Minus,
    Mult,
    Div,
    Min,
    Max,
    Assign,
    And,
    Or,
    AddAll,
    SumLong,
}

crate::obj!(Operator);

/*
sum {
        public Object apply(final Object a, Object b) {
            return NumberHelper.add((Number) a, (Number) b);
        }
    },

    /**
     * A subtraction function.
     *
     * @see NumberHelper#sub(Number, Number)
     * @since 3.0.0-incubating
     */
    minus {
        public Object apply(final Object a, final Object b) {
            return NumberHelper.sub((Number) a, (Number) b);
        }
    },

    /**
     * A multiplication function.
     *
     * @see NumberHelper#mul(Number, Number)
     * @since 3.0.0-incubating
     */
    mult {
        public Object apply(final Object a, final Object b) {
            return NumberHelper.mul((Number) a, (Number) b);
        }
    },

    /**
     * A division function.
     *
     * @see NumberHelper#div(Number, Number)
     * @since 3.0.0-incubating
     */
    div {
        public Object apply(final Object a, final Object b) {
            return NumberHelper.div((Number) a, (Number) b);
        }
    },

    /**
     * Selects the smaller of the values.
     *
     * @see NumberHelper#min(Number, Number)
     * @since 3.0.0-incubating
     */
    min {
        public Object apply(final Object a, final Object b) {
            return NumberHelper.min((Comparable) a, (Comparable) b);
        }
    },

    /**
     * Selects the larger of the values.
     *
     * @see NumberHelper#max(Number, Number)
     * @since 3.0.0-incubating
     */
    max {
        public Object apply(final Object a, final Object b) {
            return NumberHelper.max((Comparable) a, (Comparable) b);
        }
    },

    /**
     * The new incoming value (i.e. the second value to the function) is returned unchanged result in the assignment
     * of that value to the object of the {@code Operator}.
     *
     * @since 3.1.0-incubating
     */
    assign {
        public Object apply(final Object a, final Object b) {
            return b;
        }
    },

    /**
     * Applies "and" to boolean values.
     *
     * <pre>
     *     a = true, b = null -> true
     *     a = false, b = null -> false
     *     a = null, b = true -> true
     *     a = null, b = false -> false
     *     a = null, b = null -> null
     * </pre>
     *
     * @since 3.2.0-incubating
     */
    and {
        public Object apply(final Object a, final Object b) {
            if (null == a || null == b) {
                if (null == a && null == b)
                    return null;
                else
                    return null == b ? a : b;
            }
            return ((boolean) a) && ((boolean) b);
        }
    },

    /**
     * Applies "or" to boolean values.
     *
     * <pre>
     *     a = true, b = null -> true
     *     a = false, b = null -> false
     *     a = null, b = true -> true
     *     a = null, b = false -> false
     *     a = null, b = null -> null
     * </pre>
     *
     * @since 3.2.0-incubating
     */
    or {
        public Object apply(final Object a, final Object b) {
            if (null == a || null == b) {
                if (null == a && null == b)
                    return null;
                else
                    return null == b ? a : b;
            }
            return ((boolean) a) || ((boolean) b);
        }
    },

    /**
     * Takes all objects in the second {@code Collection} and adds them to the first. If the first is {@code null},
     * then the second {@code Collection} is returned and if the second is {@code null} then the first is returned.
     * If both are {@code null} then {@code null} is returned. Arguments must be of type {@code Map} or
     * {@code Collection}.
     * <p/>
     * The semantics described above for {@code Collection} are the same when applied to a {@code Map}.
     *
     * @since 3.2.0-incubating
     */
    addAll {
        public Object apply(final Object a, final Object b) {
            if (null == a || null == b) {
                if (null == a && null == b)
                    return null;
                else
                    return null == b ? a : b;
            }

            if (a instanceof Map && b instanceof Map)
                ((Map<?, ?>) a).putAll((Map) b);
            else if (a instanceof Collection && b instanceof Collection)
                ((Collection<?>) a).addAll((Collection) b);
            else
                throw new IllegalArgumentException(String.format("Objects must be both of Map or Collection: a=%s b=%s",
                        a.getClass().getSimpleName(), b.getClass().getSimpleName()));
            return a;
        }
    },

    /**
     * Sums and adds long values.
     *
     * @since 3.2.0-incubating
     */
    sumLong {
        public Object apply(final Object a, final Object b) {
            return (long) a + (long) b;
        }
    };
 */
