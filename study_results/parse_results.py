import pandas as pd


def apply_attention_check(data, should_filter):
    data['attention_check'] = data.apply(
        lambda x: x['Answer.attnCheck2.2 Minutes'], axis=1)
    data.drop(columns=['Answer.attnCheck2.2 Minutes', 'Answer.attnCheck1.1 Minute', 'Answer.attnCheck3.3 Minutes'])

    if should_filter:
        data = data[(data["steps_taken"] > 10) | (data["attention_check"] == True)]
    return data


def debug_info(data, name):
    num_rows = data.shape[0]
    print('###', name, '###')
    print('rows:', num_rows)
    print('total burgers made:', int(data['human_burgers'].sum() + data['robot_burgers'].sum()))
    print('average steps per user:', data['steps_taken'].sum() / num_rows)


db_results = pd.read_csv('data/db_game_results.csv')

nextmove_results = pd.read_csv('data/results_nextmove.csv')
nextmove_results.rename(columns=lambda c: 'participant_id' if c == 'Answer.participantId' else c, inplace=True)
nextmove_results = nextmove_results.join(db_results.set_index('participant_id'), on='participant_id')

noadvice_results = pd.read_csv('data/results_noadvice.csv')
noadvice_results.rename(columns=lambda c: 'participant_id' if c == 'Answer.participantId' else c, inplace=True)
noadvice_results = noadvice_results.join(db_results.set_index('participant_id'), on='participant_id')

# attention check
nextmove_results = apply_attention_check(nextmove_results, should_filter=True)
noadvice_results = apply_attention_check(noadvice_results, should_filter=True)

debug_info(nextmove_results, 'NextMove')
debug_info(noadvice_results, 'NoAdvice')
#print(nextmove_results.reindex(sorted(nextmove_results.columns), axis=1).to_string())
