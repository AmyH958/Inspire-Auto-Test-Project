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

WebUI.maximizeWindow()

WebUI.setText(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/input_(15trunk)_Submit'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/a_'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/a__1'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/div_'), '管理 > 流通 > 交易方式可適用使用類別')

WebUI.takeFullPageScreenshotAsCheckpoint('指參查詢')

WebUI.acceptAlert()

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/a_(Academic Reserve)'), '指參(Academic Reserve)')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/a_(Academic Reserve)'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/a_(Exhibition)'), '展示(Exhibition)')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/a_(Exhibition)'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/a_(Academic Reserve)_1'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/a_(Exhibition)'))

WebUI.takeFullPageScreenshotAsCheckpoint('展示查詢')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/a_(New Book)'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/a_(New Book)_1'), '新書(New Book)')

WebUI.takeFullPageScreenshotAsCheckpoint('新書查詢')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/a_(Normal)'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/a_(Normal)_1'), '一般(Normal)')

WebUI.takeFullPageScreenshotAsCheckpoint('一般查詢')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/a__1_2_3'))

WebUI.rightClick(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/a__1_2_3_4'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/a_testerer'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/a_eeewww'))

WebUI.scrollToElement(findTestObject('Page_(15trunk)/a__1_2_3_4_5'), 2)

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/a__1_2_3_4_5'), '修改/存檔')

WebUI.scrollToElement(findTestObject('Page_(15trunk)/a__1_2_3_4_5_6'), 1)

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/交易方式可適用使用類別/交易方式可適用使用類別查詢/a__1_2_3_4_5_6'))

WebUI.closeBrowser()

