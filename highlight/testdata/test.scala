import java.time.Instant
//^include
//     ^namespace
//               ^type
object Hello {
// ^ keyword
//       ^ type
  val x = if (true) (25 * 1.0) else "hello"
// ^keyword
//    ^variable
//         ^conditional
//             ^boolean
//                   ^number
//                        ^float
//                                    ^string
//                              ^conditional

  val y = new lower_case_intentionally
//        ^keyword.operator
//            ^type

  var mutation = "mutant"
//  ^keyword
//    ^variable

  trait Test {
// ^ keyword
//       ^ type
    def meth(i: Int)(implicit x: Boolean) = ???
//    ^keyword.function
//                       ^keyword
//                                  ^type
//        ^method
//                            ^parameter

    val anonFun: Int => Int = (a: Int) => a
  //      ^variable
  //             ^type
  //                 ^operator
  //                    ^type
  //                           ^parameter
  }

  protected abstract class Bla(test: String)
//    ^type.qualifier
//                    ^keyword
//            ^type.qualifier
//                              ^parameter
//                                     ^type

  type Hello = "25"
  // ^keyword
  //    ^type.definition
  //            ^string

  class A {
// ^ keyword
//      ^ type
    self: X =>
//  ^parameter
//        ^type
  }

  type A = { def fly(): Unit }
//            ^keyword.function
//                ^method
//                       ^type

  type A = B[({ type f[x] = M[S, x] })#f]
//               ^keyword
//                   ^type.definition

  val hello = c"some $mutation ${1}"
//            ^function.call
//                   ^punctuation.special
//                     ^variable
//                               ^number
  def meth = ???
//     ^method
  val hello2 = c"some $meth"
//                      ^method
  val hello3 = s"$$$meth$hello2%"
//               ^string
//                 ^punctuation.special
//                  ^method
//                      ^punctuation.special
//                       ^variable
//                             ^string
  val hello4 = s"$"$hello3"
//               ^string
//                 ^punctuation.special
//                  ^variable
}

//> using scala 3.1.0
//  ^keyword
//        ^parameter
//              ^string

/*
 * Beep boop
 */
// <- comment

// Single line comment
// <- comment

// Optional braces syntax
class C:
// ^keyword
//    ^type

  def test(aaaa: A): Int =
  //^keyword.function
  //  ^method
    // no curly braces, but this is still in test method
    val bbb = 1
    //^keyword
    //  ^variable
    val ccc = 2
    //^keyword
    //  ^variable
    1

object O1:
//^keyword
//     ^type

  def test: Unit = ()
  //^keyword.function
  //  ^method

object O2:
  type Elem[A] = A match
  //^keyword
  //   ^type.definition
    case String   => Char
    //^keyword
    //   ^type       ^type
    case Array[a] => a
    //   ^type       ^type

// SIP-44
class C:
// ^keyword
  //  ^type
  fooooo.map: x =>
  //     ^method.call
    x + 1

  xs.map:
    param1 =>
      param1 + 1

  foooo:
  // ^function.call
    println("")

  foooo `++`:
  //    ^operator
    val x = 1
    List(x)

  // This is an ascription
  val y = x: Int
  //         ^type

  // This is SIP-44
  val y = x:
    Int
    //^type

// Ascription expression
class C:
  foooo: Int
  //     ^type

enum Simple:
//^keyword
//   ^type
  case A, B, C
//     ^type
enum Test(a: Int) derives Codec:
// ^keyword
//    ^type
//            ^type
//                   ^keyword
//                          ^type
//    ^type
//        ^parameter
  case Test(b: String)
  // ^keyword
  //               ^type
  //      ^type
  //        ^parameter
  case Hello, Bla
  //      ^type
  //          ^type
  case Bla extends Test(256)
  //          ^keyword

opaque type Blow <: Int = 25
// ^type.qualifier
//      ^keyword
//                   ^type
//            ^type.definition

inline given Test = new Test {
// ^ storageclass
  inline def hello(inline x: Boolean) =
// ^ storageclass
//                   ^ storageclass
    inline if x then "hi" else "bye"
    // ^storageclass
    //            ^conditional
    inline x match
    // ^storageclass
      case true => 25
      case false => 26
}

object A:
// ^ keyword
//     ^type

  ::(123)
//^operator
//   ^number

object bla:
  open class Hello(A: Int)
// ^ type.qualifier
  transparent trait Hello
//   ^ type.qualifier
  infix def hello = 25
//  ^ keyword

extension (s: String)
//  ^keyword
  def test = 25
  def test2 = 25

val extension = "hello"
//     ^variable - to test "soft" keywords

given Test with
// ^keyword
//     ^type
//          ^keyword
   def test = 1
//  ^keyword.function
   val a = "hello"


class Copier:
  private val printUnit = new Printer { type PrinterType = InkJet }
  private val scanUnit = new Scanner

  export scanUnit.scan
  // ^ include
  //        ^variable
  export printUnit.{status as _, *}
  // ^ include
  //        ^variable

  def status: List[String] = printUnit.status ++ scanUnit.status
