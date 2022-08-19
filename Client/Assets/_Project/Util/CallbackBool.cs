using System;
using System.Collections.Generic;

public class CallbackBool
{
    public enum Mode
    {
        And,
        Or
    }

    private Mode _mode;
    private readonly List<Func<bool>> _callbacks = new List<Func<bool>>();
    
    public CallbackBool(Mode mode)
    {
        _mode = mode;
    }

    public void Add(Func<bool> callback)
    {
        if (_callbacks.Contains(callback))
        {
            return;
        }
        
        _callbacks.Add(callback);
    }
    
    public bool Invoke()
    {
        var value = _mode == Mode.And;
        foreach (var callback in _callbacks)
        {
            if (callback == null)
            {
                continue;
            }

            value = _mode switch
            {
                Mode.And => value && callback.Invoke(),
                Mode.Or => value || callback.Invoke(),
            };
        }

        return value;
    }
}