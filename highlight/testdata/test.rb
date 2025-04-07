require "a"
# ^ function.method.builtin

class Car < Vehicle
  # <- keyword
  #    ^ constructor

  def init(id)
    # <- keyword
    # ^ function.method

    @id = id
    # <- property
    #     ^ variable.parameter

    yield
    # <- keyword
    return
    # <- keyword
    next
    # <- keyword
  end

  private
  # ^ keyword

  public
  # ^ keyword

  protected
  # ^ keyword
end
# <- keyword

class MyClass
  #   ^ constructor

  ELEMENT8 = 8
  # ^ constant

  ELEMENT16 = 16
  # ^ constant

  def OtherClass
    #  ^ function.method

    @other.OtherClass(Something.new).inspect
    # ^ property
    #      ^ function.method
    #                 ^ constructor
    #                           ^ function.method
    #                                ^ function.method
  end
end

if __FILE__ == $0
  # ^ constant.builtin
end

"hello"
# ^ string
%(hello)
# ^ string
%w(hello goodbye)
#   ^ string
#         ^ string

:hello
# ^ string.special.symbol
%s(hello)
# ^ string.special.symbol
%I(hello goodbye)
#  ^ string.special.symbol
#         ^ string.special.symbol

/hello/
# ^ string.special.regex
%r(hello)
# ^ string.special.regex

12345
# ^ number
12.345
# ^ number

nil
# ^ constant.builtin
true
# ^ constant.builtin
TRUE
# ^ constant
false
# ^ constant.builtin
FALSE
# ^ constant

expr = [false | __LINE__ | __FILE__ | __ENCODING__]
#       ^ constant.builtin
#               ^ constant.builtin
#                          ^ constant.builtin
#                                     ^ constant.builtin

case expr
  in {a: 5, b:, **nil}
#               ^ operator
#                 ^ constant.builtin
  in [false | __LINE__ | __FILE__ | __ENCODING__]
#     ^ constant.builtin
#             ^ constant.builtin
#                        ^ constant.builtin
#                                   ^ constant.builtin
  else
end

one = 1
# <- variable

def two()
  three = one
  # ^ variable
  #        ^ function.method

  four = three
  # ^ variable
  #       ^ variable

  four.each do |i|
    puts i, three, five
    #    ^ variable.parameter
    #        ^ variable (because blocks are closures)
    #               ^ function.method
  end

  four.each do |(a, b), c: d, e = f|
    #            ^ variable.parameter
    #               ^ variable.parameter
    #                   ^ variable.parameter
    #                      ^ function.method
    #                         ^ variable.parameter
    #                             ^ function.method
    puts a, b, c, d, e, f
    #    ^ variable.parameter
    #       ^ variable.parameter
    #          ^ variable.parameter
    #             ^ function.method
    #                ^ variable.parameter
    #                   ^ function.method
  end

  five ||= 1
  # ^ variable

  six -> (seven) { eight(seven, five) }
  # ^ function.method
  #        ^ variable.parameter
  #                 ^ function.method
  #                      ^ variable.parameter
  #                              ^ variable

  seven
  # ^ function.method (because the `seven` above was in the scope of the lambda)
end
