import React from "react";
import "./App.css";
import { Observable, Subject, Subscription, combineLatest, map } from "rxjs";

const a$ = new Subject<number>();
const b$ = new Subject<number>();
const c$ = combineLatest([a$, b$]).pipe(
    map(([a, b]) => {
        return a + b;
    })
);
const d$ = a$.pipe(map((val) => val * 2 + 1));
const dBackward$ = new Subject<number>();
dBackward$.subscribe((val) => {
    a$.next((val - 1) / 2);
});

//a$.subscribe((val) => {
//    console.log("a", val);
//});
//b$.subscribe((val) => {
//    console.log("b", val);
//});
//c$.subscribe((val) => {
//    console.log("c", val);
//});
//d$.subscribe((val) => {
//    console.log("d", val);
//});
//dBackward$.subscribe((val) => {
//    console.log("dBackward", val);
//});

function App() {
    React.useEffect(() => {
        a$.next(1);
        b$.next(4);
    }, []);

    return (
        <>
            <h2>Functional Reactive Programming Example</h2>
            <table>
                <thead>
                    <tr>
                        <th>A</th>
                        <th>B</th>
                        <th>C = A+B</th>
                        <th>D = 2*A + 1</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <InputBox reactive$={a$} />
                        </td>
                        <td>
                            <InputBox reactive$={b$} />
                        </td>
                        <td>
                            <InputBox observable$={c$} />
                        </td>
                        <td>
                            <InputBox observable$={d$} reactive$={dBackward$} />
                        </td>
                    </tr>
                </tbody>
            </table>
        </>
    );
}

export default App;

function InputBox({
    reactive$,
    observable$,
}: {
    reactive$?: Subject<number>;
    observable$?: Observable<number>;
}) {
    const [value, setValue] = React.useState("0");
    const [dirty, setDirty] = React.useState(false);

    React.useEffect(() => {
        if (!reactive$ && !observable$) {
            throw new Error("reactive$ and observable$ cannot both be null");
        }
        let sub: Subscription;
        if (observable$) {
            sub = observable$.subscribe((val) => {
                setValue(val.toString());
            });
        } else if (reactive$) {
            sub = reactive$.subscribe((val) => {
                setValue(val.toString());
            });
        }

        return () => {
            sub.unsubscribe();
        };
    }, []);

    function updateReactive() {
        reactive$?.next(parseInt(value));
        setDirty(false);
    }

    return (
        <input
            style={{ backgroundColor: dirty ? "yellow" : "white" }}
            type="text"
            value={value}
            onChange={(e) => {
                if (observable$ && !reactive$) {
                    // If there is an observable and no reactive, then we are in read-only mode
                    return;
                }
                setValue(e.target.value);
                setDirty(true);
            }}
            onBlur={() => {
                updateReactive();
            }}
            onKeyDown={(e) => {
                if (e.key === "Enter") {
                    updateReactive();
                }
            }}
        />
    );
}
