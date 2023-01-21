import pandas as pd


def apply_attention_check(data, should_filter):
    data['attention_check'] = data.apply(
        lambda x: x['Answer.attnCheck2.2 Minutes'], axis=1)
    data.drop(columns=['Answer.attnCheck2.2 Minutes', 'Answer.attnCheck1.1 Minute', 'Answer.attnCheck3.3 Minutes'], inplace=True)

    if should_filter:
        data = data[(data["steps_taken"] > 10) | (data["attention_check"] is True)]
    return data


def bool_to_likert(bool1, bool2, bool3, bool4, bool5):
    if bool1:
        return 1
    elif bool2:
        return 2
    elif bool3:
        return 3
    elif bool4:
        return 4
    elif bool5:
        return 5
    else:
        return None


def apply_likert_scale(data, name, new_name):
    data[new_name] = data.apply(
        lambda x: bool_to_likert(x['Answer.%s1.1' % name],
                                 x['Answer.%s2.2' % name],
                                 x['Answer.%s3.3' % name],
                                 x['Answer.%s4.4' % name],
                                 x['Answer.%s5.5' % name]), axis=1)
    data.drop(columns=['Answer.%s1.1' % name, 'Answer.%s2.2' % name, 'Answer.%s3.3' % name, 'Answer.%s4.4' % name, 'Answer.%s5.5' % name],
              inplace=True)
    return data


def apply_all_scales(data, scales):
    for (name, new_name) in scales:
        data = apply_likert_scale(data, name, new_name)
    return data


def debug_info(data, name):
    num_rows = data.shape[0]
    print('###', name, '###')
    print('rows:', num_rows)
    print('total burgers made:', int(data['human_burgers'].sum() + data['robot_burgers'].sum()))
    print('average steps per user:', data['steps_taken'].sum() / num_rows)


def write_bonus_file(data, name):
    bonus_per_burger = 0.1
    data = data[(data["human_burgers"] > 0) | (data["robot_burgers"] > 0)]
    data['usd'] = data[['human_burgers', 'robot_burgers']].sum(axis=1)
    data['usd'] *= bonus_per_burger
    bonus_data = data.filter(['WorkerId', 'AssignmentId', 'usd'], axis=1)
    bonus_data = bonus_data.round(2)
    bonus_data.to_csv('data/%s_bonus.csv' % name)


db_results = pd.read_csv('data/db_game_results.csv')

nextmove_results = pd.read_csv('data/results_nextmove.csv')
nextmove_results.rename(columns=lambda c: 'participant_id' if c == 'Answer.participantId' else c, inplace=True)
nextmove_results = nextmove_results.join(db_results.set_index('participant_id'), on='participant_id')

noadvice_results = pd.read_csv('data/results_noadvice.csv')
noadvice_results.rename(columns=lambda c: 'participant_id' if c == 'Answer.participantId' else c, inplace=True)
noadvice_results = noadvice_results.join(db_results.set_index('participant_id'), on='participant_id')

# filter results with failed attention check and less than 10 moves in the game
nextmove_results = apply_attention_check(nextmove_results, should_filter=True)
noadvice_results = apply_attention_check(noadvice_results, should_filter=True)

# remove unneeded columns
unneeded = ['AcceptTime', 'ApprovalTime', 'Approve', 'AssignmentDurationInSeconds', 'AssignmentStatus', 'AutoApprovalDelayInSeconds',
            'AutoApprovalTime', 'CreationTime', 'Description', 'Expiration', 'HITId', 'HITTypeId', 'Keywords', 'Last30DaysApprovalRate',
            'Last7DaysApprovalRate', 'LifetimeApprovalRate', 'LifetimeInSeconds', 'MaxAssignments', 'NumberOfSimilarHITs', 'Reject', 'RejectionTime',
            'RequesterAnnotation', 'RequesterFeedback', 'Reward', 'SubmitTime', 'Title']
nextmove_results.drop(columns=unneeded, inplace=True)
noadvice_results.drop(columns=unneeded, inplace=True)

# apply likert scale transformation
likert_scales = [('agitated', 'AgitatedCalm'),
                 ('anxious', 'AnxiousRelaxed'),
                 ('foolish', 'FoolishSensible'),
                 ('ignorant', 'IgnorantKnowledgeable'),
                 ('incompetent', 'IncompetentCompetent'),
                 ('incompliant', 'IncompliantCompliant'),
                 ('irresponsible', 'IrresponsibleResponsible'),
                 ('predict', 'UnpredictablePredictable'),
                 ('quiescent', 'QuiescentSurprised'),
                 ('restrictive', 'UnrestrictiveRestrictive'),
                 ('unintelligent', 'UnintelligentIntelligent'),
                 ]
nextmove_results = apply_all_scales(nextmove_results, likert_scales)
noadvice_results = apply_all_scales(noadvice_results, likert_scales)

# write bonus file
# write_bonus_file(nextmove_results, 'nextmove')
# write_bonus_file(noadvice_results, 'noadvice')

debug_info(nextmove_results, 'NextMove')
debug_info(noadvice_results, 'NoAdvice')
# print(nextmove_results.reindex(sorted(nextmove_results.columns), axis=1).to_string())
print(nextmove_results.to_string())
