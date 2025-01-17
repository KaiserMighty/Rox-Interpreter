package interpreter;

import java.util.List;

interface RoxCallable
{
    int arity();
    Object call(Interpreter interpreter, List<Object> arguments);
}