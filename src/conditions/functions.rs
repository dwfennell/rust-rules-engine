use serde::{Deserialize, Serialize};

use super::ConditionValue;

#[derive(Debug, Serialize, Deserialize)]
pub enum Fn {
    // - Logical -
    And(Vec<ConditionValue>),
    Or(Vec<ConditionValue>),
    Not(Box<ConditionValue>),
    //False()
    //If(test, true_value, false_value)
    //Switch(exp, val1, val2, ...)
    //Xor(a, b, ...)

    // - Comparison -
    GreaterThan(Vec<ConditionValue>),
    LessThan(Vec<ConditionValue>),
    Equal(Vec<ConditionValue>),

    // - Date and time -
    //Date(y, m, d)
    //DateDif(a, b, unit)
    //DateValue(text)
    //Day(date)
    //Days(end, start)
    //days360
    //EDate(date, months)
    //EOMonth(date, months)
    //Hour(serial_time)
    //ISOWeekNum(date)
    //Minute(serial_number)
    //Month(serial_number)
    //NetWorkDays(start, end, holidays)
    //NetWorkDaysIntl(start, end, weekend, holidays)
    //Now()
    //Second(serial_number)
    //Time(hour, minute, second)
    //TimeValue(text)
    //Today()
    //WeekDay(serial_number, return_type)
    //WeekNum(serial_number, return_type)
    //WorkDay(start, days, holidays)
    //WorkDayIntl(start, days, weekend, holidays)
    //Year(date)
    //YearFrac(start, end, basis)

    // - Lookup and Reference
    // NA?

    // - Text -
    //Char(number)
    //Clean(text)
    //Code(text)
    //Concat(a, b, ...)
    //Concatenate(a, b, c, ...)
    //Dollar(number, decimals)
    //Exact(a, b)
    //Find(find, within, start_num)
    //Fixed(number, decimals, no_commas)
    //Left(text, num_chars)
    //Len(text)
    //Lower(text)
    //Mid(text, start, end)
    //NumberValue(text, decimal_separate, group_separator)
    //Proper(text)
    //Replace(old, start, num_chars, new_text)
    //Rept(text, num)
    //Right(text, num_chars)
    //Search(find, within, start)
    //Substitute(text, old, new, instance)
    //Text(value, format_text)
    //TextJoin(delimiter, ingore_empty, text1, text2, ...)
    //Trim(text)
    //Unichar(number)
    //Unicode(text)
    //Upper(text)
    //Value(text)

    // - Dynamic array -
    // NA?

    // - Engineering -
    //Bin2Dec(num)
    //Bin2Hex(num)
    //BitAnd(a, b)
    //BitLShift(num, shift_amt)
    //BitOr(a, b)
    //BitRShift(num, shift_amt)
    //BitXOr(a, b,)
    //Complex(real_num, i_num, suffix)
    //Convert(number, from, to)
    //Dec2Bin(number, places)
    //Dec2Oct(number, places)
    //Delta(a, b)
    //Hex2Bin(number, places)
    //Hex2Dec(number)
    //Hex2Oct(number)
    //Imabs(i_number)
    //Imaginary(i_number)
    //ImPower(i_number, number)
    //ImProduct(i_a, i_b, ...)
    //ImReal(i_number)
    //ImSub(i_number, i_number)

    // - Financial -
    //AccrInt
    //AccrIntm
    //AmorDeGrc
    //AmorLinc
    //CoupDayBs
    //CoupDays
    //CoupDaySnc
    //COUPNCD
    //COUPNUM
    //COUPPCD
    //CUMIPMT
    //CUMPRINC
    //DB
    //DDB
    //DISC
    //DOLLARDE
    //DOLLARFR
    //DURATION
    //EFFECT
    //FV
    //FVSCHEDULE
    //INTRATE
    //IPMT
    //IRR
    //ISPMT
    //MDURATION
    //MIRR
    //NOMINAL
    //NPER
    //NPV
    //ODDFPRICE
    //ODDFYIELD
    //ODDLPRICE
    //ODDLYIELD
    //PDURATION
    //PMT
    //PPMT
    //PRICE
    //PRICEDISC
    //PRICEMAT
    //PV
    //RATE
    //RECEIVED
    //RRI
    //SLN
    //SYD
    //TBILLEQ
    //TBILLPRICE
    //TBILLYIELD
    //VDB
    //XIRR
    //XNPV
    //YIELD
    //YIELDDISC
    //YIELDMAT

    // - Information -
    // Incomplete: NA?
    //IsEven(val)
    //IsLogical(val)
    //IsNonText(val)
    //IsNumber(val)
    //IsOdd(val)
    //IsText(val)
    //N(val)
    //Type(val)
    //T(value)

    // - Math -
    //Abs(num)
    //Aggregate(function_num, options, ref1, ref2)
    //Arabic(roman_text)
    //Base(number, radix, min_length)
    //Ceiling(num, significance)
    //Ceiling.Math(number, significance, mode)
    //Ceiling.Precise(number, significance)
    //Combin(number, number_chosen)
    //CombinA(number, number_chose)
    //Decimal(number, radix)
    //Even(number)
    //Exp(number)
    //Fact(number)
    //FactDouble(number)
    //Floor(number, significance)
    //Floor.Math(number, significance, mode)
    //Floor.Precise(number, significance)
    //Gcd(a, b, ...)
    //Int(number)
    //LCM(a, b, c, ...)
    //Ln(number)
    //Log(number)
    //Log10(number, base)
    //MDeterm(array)
    //MinVerse(array)
    //MMult(array_a, array_b)
    //Mod(number, divisor)
    //MRound(number, significance)
    //MUnit(dimension)
    //Odd(number)
    //Pi
    //Power(number, power)
    //Product(a, b, ...)
    //Quotient(numerator, denominator)
    //Rand()
    //RandBetween(bottom, top)
    //Roman(number, from)
    //Round(number, num_digits)
    //RoundDown(number, num_digits)
    //RoundUp(number, num_digits)
    //Sign(number)
    //Sqrt(number)
    //Subtotal(function_num, ref1, ref2, ...)
    //Sum(a, b, ...)
    //SumIf(range, criteria, sum_range)
    //SumIfs() ...
    //SumProduct(array1, array2, ...)
    //SumSq(a, b, ...)
    //SumX2My2(array_x, array_y)
    //SumX2Py2(array_x, array_y)
    //SumXMy2(array_x, array_y)
    //Trunc(number, num_digits)

    // - Trigonometry -
    //ACos(number)
    //ASin(number)
    //ATan(number)
    //ATan2(x, y)
    //Cos(number)
    //CosH(number)
    //Cot(number)
    //Csc(number)
    //Degrees(angle)
    //Radians(angle)
    //Sec(number)
    //Sin(number)
    //SinH(number)
    //Tan(number)

    // - Statistical -
    //AveDev(a, b, ...)
    //Average(a, b, ...)
    //AverageA(a, b, ...)
    //AverageIf(range, criteria, average_range)
    //AverageIfs
    //BINOM.dist
    //BinomDist
    //Count(a, b, ...)
    //CountA(a, b, ...)
    //CountBlank(a, b, ...)
    //CountIf(range, criteria)
    //CountIfs
    //DevSq(a, b, ...)
    //Forecast(x, known_ys, known_xs)
    //Forecast.ets
    //Forecast.ets.Confint
    //Forecast.ets.seasonality
    //Forecast.ets.stat
    //Forecast.linear
    //Frequency(data_array, bins_array)
    //GeoMean(a, b, ...)
    //Harmean(a, b, ...)
    //Intercept(known_ys, known_xs)
    //Large(array, k)
    //Linest(known_ys, known_xs, const, stats)
    //MaxA(a, b, ...)
    //MaxIfs
    //Median(a, b, ...)
    Min(Vec<ConditionValue>),
    Max(Vec<ConditionValue>),
    //MinA(a, b, ...)
    //MinIfs
    //Mode(a, b, ...)
    //Mode.Mult((a, b, ...))
    //Mode.Sngl(a, b, ...)
    //Norm.dist(x, mean, standard_dev, cumulative)
    //Norm.inv(probability, mean, standard_dev)
    //Norm.s.dist(z, cumulative)
    //Percentile(array, k)
    //Percentile.inc(array, k)
    //Percentrank(array, x, significance)
    //Percentrank.exc(array, x, significance)
    //Percentrank.inc(array, x, significance)
    //Permut(number, number_chosen)
    //PermutationA(number, number_chosen)
    //Quartile(array, quart)
    //Quartile.exc(array, quart)
    //Quartile.inc(array, quart)
    //Rank(number, ref, order)
    //Rank.Avg(number, ref, order)
    //Rank.Eq(number, ref, order)
    //Skew(a, b, ...)
    //Skew.P(a, b, ...)
    //Slope(know_ys, known_xs)
    //Small(array, k)
    //Standardize(x, mean, standard_dev)
    //Stdev(a, b, ...)
    //Stdev.p(a, b, ...)
    //Stdev.s(a, b, ...)
    //stdevA(a, b, ...)
    //Stdevp(a, b, ...)
    //Stdevpa(a, b, ...)
    //TrimMean(array, percent)
    //var(a, b, ...)
    //var.p(a, b, ...)
    //var.s(a, b, ...)
    //vara(a, b, ...)
    //varp(a, b, ...)
    //varpa(a, b, ...)

    // - web -
    //encodeUrl(text)
    //filterxml(xml, xpath)
    //webservice(url)

    // - Database -
    // NA?
}
