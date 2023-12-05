using System;
using System.IO;
using System.IO.Pipes;
using System.Collections.Generic;
using System.Threading;
using System.Windows;
using System.Windows.Automation;
using System.Windows.Forms;
using Newtonsoft.Json;
using System.ComponentModel;

public static class RectExtensions
{
    public static int[] ToIntArray(this Rect rect)
    {
        return
        [
            (int)Math.Round(rect.X),
            (int)Math.Round(rect.Y),
            (int)Math.Round(rect.Width),
            (int)Math.Round(rect.Height)
        ];
    }
}

class Program
{
    static void Main(string[] args)
    {
        // // diagnostic mode
        // using (StreamWriter sw = new StreamWriter(Console.OpenStandardOutput()))
        // {
        //     while (true)
        //     {
        //         PrintUnderMouse(sw);
        //         Thread.Sleep(100);
        //     }
        // }
        using (NamedPipeServerStream pipeServer = new NamedPipeServerStream("testpipe"))
        {
            Console.WriteLine("NamedPipeServerStream object created.");

            // Wait for a client to connect
            Console.Write("Waiting for client connection...");
            pipeServer.WaitForConnection();

            Console.WriteLine("Client connected.");

            using (StreamWriter sw = new StreamWriter(pipeServer))
            {
                while (true)
                {
                    PrintUnderMouse(sw);
                    Thread.Sleep(100);
                }
            }
        }
    }

    static void PrintUnderMouse(StreamWriter sw)
    {
        int cursorX = Cursor.Position.X;
        int cursorY = Cursor.Position.Y;
        AutomationElement element = null;
        try
        {
            element = AutomationElement.FromPoint(new Point(cursorX, cursorY));
        }
        catch (Win32Exception e)
        {
            if (e.NativeErrorCode == 5)
            { // access denied, do nothing
                return;
            }
            else
            {
                Console.WriteLine(e);
            }
        }
        if (element == null) return;
        var outputData = new
        {
            cursorPosition = new int[] { cursorX, cursorY },
            elementDetails = DetailsFor(element),
            interestingElements = GatherInteresting(element).Select(e =>
            {
                (var elem, var depth, var relationship) = e;
                var details = DetailsFor(elem);
                return new
                {
                    details,
                    depth,
                    relationship,
                };
            }),
        };

        string json = JsonConvert.SerializeObject(outputData);
        sw.WriteLine(json);
        sw.Flush();
    }

    static object DetailsFor(AutomationElement elem)
    {
        int[] boundingRect = ((Rect)elem.GetCurrentPropertyValue(AutomationElement.BoundingRectangleProperty)).ToIntArray();
        string name = elem.Current.Name;
        string controlType = elem.Current.ControlType.ProgrammaticName;
        string className = elem.Current.ClassName;
        string automationId = elem.Current.AutomationId;
        string value = GetValue(elem);  // Extracted value fetching to a separate method
        return new
        {
            name,
            boundingRect,
            controlType,
            className,
            automationId,
            value,
        };
    }


    static string GetValue(AutomationElement element)
    {
        object patternObj;
        if (element.TryGetCurrentPattern(ValuePattern.Pattern, out patternObj))
        {
            ValuePattern valuePattern = patternObj as ValuePattern;
            return valuePattern.Current.Value;
        }
        return null;  // or "N/A" or any other default value
    }

    static IEnumerable<(AutomationElement, int, string)> GatherInteresting(AutomationElement element, int maxDepth = 2)
    {
        // Yield children of the element
        foreach (var child in GetDescendants(element, 0, maxDepth))
        {
            yield return (child.Item1, child.Item2, "Child");
        }

        // Yield siblings of the element
        TreeWalker walker = TreeWalker.ControlViewWalker;
        AutomationElement sibling = walker.GetNextSibling(element);
        while (sibling != null)
        {
            yield return (sibling, 0, "Sibling");
            sibling = walker.GetNextSibling(sibling);
        }

        // Yield siblings of the element's parent
        AutomationElement parent = walker.GetParent(element);
        if (parent != null)
        {
            AutomationElement parentSibling = walker.GetNextSibling(parent);
            while (parentSibling != null)
            {
                yield return (parentSibling, 0, "Parent's Sibling");
                parentSibling = walker.GetNextSibling(parentSibling);
            }
        }
    }

    static IEnumerable<AutomationElement> GetAncestors(AutomationElement element)
    {
        TreeWalker walker = TreeWalker.ControlViewWalker;
        AutomationElement parentElement = walker.GetParent(element);

        while (parentElement != null)
        {
            yield return parentElement;
            parentElement = walker.GetParent(parentElement);
        }
    }

    static IEnumerable<(AutomationElement, int)> GetDescendants(AutomationElement element, int currentDepth, int maxDepth)
    {
        if (currentDepth > maxDepth)
        {
            yield break;
        }

        TreeWalker walker = TreeWalker.ControlViewWalker;
        AutomationElement childElement = null;
        try
        {
            childElement = walker.GetFirstChild(element);
        }
        catch (Exception e)
        {
            Console.WriteLine($"Encountered error gathering descendants on {DetailsFor(element)}");
            Console.WriteLine(e);
        }

        while (childElement != null)
        {
            yield return (childElement, currentDepth);

            foreach (var grandChild in GetDescendants(childElement, currentDepth + 1, maxDepth))
            {
                yield return grandChild;
            }

            try
            {
                childElement = walker.GetNextSibling(childElement);
            }
            catch (Exception e)
            {
                Console.WriteLine($"Gathering siblings failed on {DetailsFor(childElement)}");
                Console.WriteLine(e);
            }
        }
    }
}

