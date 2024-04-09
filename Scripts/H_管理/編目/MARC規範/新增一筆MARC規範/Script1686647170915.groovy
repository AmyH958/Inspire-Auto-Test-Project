import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://10.11.20.15/inspireapp/logon')

WebUI.enableSmartWait()

WebUI.maximizeWindow()

WebUI.setText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/input_(15trunk)_Submit'))

WebUI.scrollToElement(findTestObject('管理/編目/新增一筆MARC規範/a_'), 2)

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/a_'), '管理')

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/a_'))

WebUI.scrollToElement(findTestObject('管理/編目/新增一筆MARC規範/a__1'), 2)

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/a__1'), '編目')

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/a__1'))

WebUI.scrollToElement(findTestObject('管理/編目/新增一筆MARC規範/a_MARC'), 2)

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/a_MARC'), 'MARC規範')

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/a_MARC'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/div_MARC'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/div_MARC'), '管理 > 編目 > MARC規範')

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/a_ (1)'), '新增', FailureHandling.CONTINUE_ON_FAILURE)

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/a_ (1)'))

WebUI.setText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/input__valoaretag'), '100')

WebUI.setText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/input__eticheta'), '測試用20230613')

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/input__Checkbox'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/input__Checkbox_0'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/input__tagPatternValidare'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/input__Checkbox_1'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/input__TextField_0'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/a__1_2'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/div_'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/div_'), '存檔成功')

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/span_'), '確定')

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/span_'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/span__1'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/span__1'), '子分欄', FailureHandling.CONTINUE_ON_FAILURE)

WebUI.scrollToElement(findTestObject('管理/編目/新增一筆MARC規範/a__1_2_3'), 1, FailureHandling.CONTINUE_ON_FAILURE)

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/a__1_2_3'), ' 新增 ')

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/a__1_2_3'))

WebUI.setText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/input__subtag'), 'a')

WebUI.setText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/input__eticheta'), '測試用')

WebUI.setText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/input__TextField_0'), '9999')

WebUI.setText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/input_Link picklist_prefix'), '1')

WebUI.setText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/input_Link picklist_sufix'), '9')

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/select_AcquisitionAuCallNumberGeneratorAuth_b0a976'), 
    '0', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/select_AcquisitionAuCallNumberGeneratorAuth_b0a976'), 
    '1', true)

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/input__Checkbox'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/input__Checkbox_0'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/input__Checkbox_1'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/a__1_2_3_4'), ' 修改/存檔 ', FailureHandling.CONTINUE_ON_FAILURE)

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/a__1_2_3_4'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/div_'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/div_'), '存檔成功')

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/span_'), '確定')

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/span_'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/span__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/span__1_2'), '子分欄驗證')

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/a__1_2_3_4_5'), '新增', FailureHandling.CONTINUE_ON_FAILURE)

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/a__1_2_3_4_5'))

WebUI.setText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/input__cod'), '1')

WebUI.setText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/input__etichetaValoriIndicator'), '測試用20230613')

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/a__1_2_3_4_5_6'), '修改/存檔')

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/a__1_2_3_4_5_6'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/div_'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/div_'), '存檔成功')

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/a__1_2_3_4_5_6_7'), '確定')

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/span_'))

WebUI.takeFullPageScreenshotAsCheckpoint('新增成功')

WebUI.click(findTestObject('Object Repository/管理/編目/新增一筆MARC規範/a__1_2_3_4_5_6_7_8'))

WebUI.closeBrowser()

