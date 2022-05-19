import random
import time
from turtle import color

import matplotlib.pyplot as plt
import pandas as pd
from catastrophe_risk_modelling import construct_model


def py_construct_model(event_ids):
    vulnerablilities = pd.read_csv("./vulnerability.csv")
    foot_print = pd.read_csv('./footprint.csv')
    event_ids = pd.DataFrame(event_ids)

    model = pd.merge(event_ids, foot_print, how="inner", on="event_id")

    model.rename(columns={
        "probability": "footprint_probability"
    }, inplace=True)

    model = pd.merge(model, vulnerablilities,
                     how="inner", on="intensity_bin_id")

    model.rename(columns={
        "probability": "vulnerability_probability"
    }, inplace=True)

    model['total_probability'] = model['footprint_probability'] * \
        model['vulnerability_probability']

    return model


def generate_event_ids_for_python(numbers_of_events):
    return [{
        "event_id": random.randint(1, 4)
    } for _ in range(0, numbers_of_events)]


def generate_event_ids_for_rust(numbers_of_events):
    return [random.randint(1, 4)
            for _ in range(0, numbers_of_events)]


if __name__ == "__main__":
    x = []
    python_y = []
    rust_y = []

    for i in range(10, 3000, 10):
        x.append(i)

        python_event_ids = generate_event_ids_for_python(i)
        rust_event_ids = generate_event_ids_for_rust(i)

        python_start = time.time()
        py_construct_model(event_ids=python_event_ids)
        python_end = time.time()
        python_y.append(python_end - python_start)

        rust_start = time.time()
        construct_model(event_ids=rust_event_ids)
        rust_end = time.time()
        rust_y.append(rust_end - rust_start)

    plt.plot(x, python_y, color="red")
    plt.plot(x, rust_y, color="green")
    plt.savefig('catastrophe_risk_model_benchmark.png')
